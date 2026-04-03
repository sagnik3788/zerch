use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer};
use chrono::Local;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use zerch_core::cosine_similarity;
use zerch_embed::LocalEmbedder;
use zerch_storage::VectorStore;
use reqwest::Client;

mod summarize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: String,
    pub ts: String,
    pub level: String,
    pub service: String,
    pub msg: String,
    pub similarity: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    pub success: bool,
    pub logs: Vec<LogEntry>,
    pub count: usize,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub success: bool,
    pub query: String,
    pub results: Vec<SearchResult>,
    pub count: usize,
    pub message: String,
}

fn extract_log_level(line: &str) -> String {
    let lower = line.to_lowercase();
    if lower.contains("error") || lower.contains("err") {
        "ERROR".to_string()
    } else if lower.contains("warn") || lower.contains("warning") {
        "WARN".to_string()
    } else if lower.contains("debug") {
        "DEBUG".to_string()
    } else {
        "INFO".to_string()
    }
}

fn extract_timestamp(_line: &str) -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

fn extract_service(line: &str) -> String {
    if let Some(start) = line.find('[') {
        if let Some(end) = line.find(']') {
            if end > start + 1 {
                return line[start + 1..end].to_string();
            }
        }
    }
    "unknown".to_string()
}

fn search_vectors(
    query: &str,
    embedder: &mut LocalEmbedder,
    store_path: &str,
    limit: usize,
) -> Result<Vec<SearchResult>, String> {
    // Embed the query
    let query_vector = embedder.embed(query).map_err(|e| format!("Embed error: {}", e))?;

    let mut file = File::open(store_path).map_err(|e| format!("File error: {}", e))?;
    let mut top_matches: Vec<(f32, String, usize)> = Vec::new();
    let mut result_id = 0;

    // Loop over each vector+text and compute similarity
    loop {
        // Read the vector length
        let mut len_buf = [0u8; 4];
        if file.read_exact(&mut len_buf).is_err() {
            break;
        }
        let vec_len = u32::from_le_bytes(len_buf) as usize;

        // Read vector data
        let mut vec_bytes = vec![0u8; vec_len * 4];
        if file.read_exact(&mut vec_bytes).is_err() {
            break;
        }

        // Convert bytes to f32 vector
        let mut log_vector = Vec::with_capacity(vec_len);
        for chunk in vec_bytes.chunks_exact(4) {
            let bytes: [u8; 4] = chunk.try_into().unwrap();
            log_vector.push(f32::from_le_bytes(bytes));
        }

        // Compute cosine similarity
        let score = cosine_similarity(&query_vector, &log_vector).score;

        // Read text length
        let mut len_buf = [0u8; 4];
        if file.read_exact(&mut len_buf).is_err() {
            break;
        }
        let text_len = u32::from_le_bytes(len_buf) as usize;

        // Read text bytes
        let mut text_bytes = vec![0u8; text_len];
        if file.read_exact(&mut text_bytes).is_err() {
            break;
        }
        let text = String::from_utf8_lossy(&text_bytes).into_owned();

        // Track top results
        top_matches.push((score, text, result_id));
        top_matches.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        top_matches.truncate(limit);

        result_id += 1;
    }

    // Convert to SearchResult format
    let results = top_matches
        .into_iter()
        .map(|(score, text, id)| SearchResult {
            id: format!("result-{}", id),
            score,
            text,
        })
        .collect();

    Ok(results)
}

#[post("/api/upload")]
async fn upload_logs(
    mut payload: Multipart,
    embedder: web::Data<Arc<Mutex<LocalEmbedder>>>,
    store: web::Data<Arc<VectorStore>>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut logs = Vec::new();
    let mut total_indexed = 0u32;

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let field_name = field.name().to_string();

        if field_name == "file" {
            let mut file_content = Vec::new();
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                file_content.extend_from_slice(&data);
            }

            let content_str = String::from_utf8_lossy(&file_content);
            let lines: Vec<&str> = content_str.lines().collect();

            log::info!("Processing file with {} lines", lines.len());

            for (idx, line) in lines.iter().enumerate() {
                if line.trim().is_empty() {
                    continue;
                }

                let level = extract_log_level(line);
                let ts = extract_timestamp(line);
                let service = extract_service(line);
                let msg = line.to_string();

                // Embed the log line using the Rust engine
                let vector = match embedder.lock() {
                    Ok(mut emb) => match emb.embed(&msg) {
                        Ok(vec) => vec,
                        Err(e) => {
                            log::error!("Failed to embed log: {}", e);
                            vec![0.5; 384]
                        }
                    },
                    Err(e) => {
                        log::error!("Failed to lock embedder: {}", e);
                        vec![0.5; 384]
                    }
                };

                // Store the vector and text in the vector store
                if let Err(e) = store.append_vector(&vector, &msg) {
                    log::error!("Failed to store vector: {}", e);
                }

                let log_entry = LogEntry {
                    id: format!("log-{}-{}", Uuid::new_v4(), idx),
                    ts,
                    level,
                    service,
                    msg,
                    similarity: 0.75,
                };

                log::info!("[{}] Indexed: {}", total_indexed + 1, log_entry.msg);
                logs.push(log_entry);
                total_indexed += 1;
            }
        }
    }

    let response = UploadResponse {
        success: true,
        logs,
        count: total_indexed as usize,
        message: format!("Successfully indexed {} logs", total_indexed),
    };

    log::info!("Upload complete: {} logs indexed", total_indexed);
    Ok(HttpResponse::Ok().json(response))
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "engine": "Running (Rust)"
    }))
}

#[get("/api/search")]
async fn search(
    query: web::Query<SearchQuery>,
    embedder: web::Data<Arc<Mutex<LocalEmbedder>>>,
    store: web::Data<Arc<VectorStore>>,
) -> Result<HttpResponse, actix_web::Error> {
    let search_query = query.q.trim().to_string();

    if search_query.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "message": "Search query cannot be empty"
        })));
    }

    let limit = query.limit.unwrap_or(5);
    let store_path = store.path.to_string_lossy().to_string();
    let embedder_arc = embedder.clone();
    let query_copy = search_query.clone();

    // Search in a blocking context to avoid blocking async runtime
    let search_results = web::block(move || {
        let mut guard = embedder_arc.lock().map_err(|e| {
            log::error!("Lock failed: {}", e);
            format!("Lock error: {}", e)
        })?;

        search_vectors(&query_copy, &mut guard, &store_path, limit)
    })
    .await
    .map_err(|e| {
        log::error!("Blocking task error: {}", e);
        actix_web::error::ErrorInternalServerError("Task error")
    })?
    .map_err(|e| {
        log::error!("Search error: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let count = search_results.len();
    let response = SearchResponse {
        success: true,
        query: search_query,
        results: search_results,
        count,
        message: format!("Found {} matching logs", count),
    };

    log::info!("Search complete: {} results", count);
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok(); 

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Loading embedding model... this might take a moment.");
    let embedder = match LocalEmbedder::load() {
        Ok(emb) => {
            log::info!("Model loaded successfully");
            Arc::new(Mutex::new(emb))
        }
        Err(e) => {
            log::error!("Failed to load model: {}", e);
            panic!("Cannot start server without embedder model");
        }
    };

    let store = Arc::new(VectorStore::new("zerch_data.bin"));
    let client = Client::new();

    log::info!("Starting Zerch API server on http://127.0.0.1:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(web::Data::new(embedder.clone()))
            .app_data(web::Data::new(store.clone()))
            .app_data(web::Data::new(client.clone()))
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(health)
            .service(upload_logs)
            .service(search)
            .service(summarize::summarize)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
