use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer};
use chrono::Local;
use futures_util::stream::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use zerch_embed::LocalEmbedder;
use zerch_storage::QdrantStore;

mod summarize;

// ---------------------------------------------------------------------------
// Data models
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: String,
    pub ts: String,
    pub level: String,
    pub service: String,
    pub msg: String,
    pub template: String,
    pub params: Vec<ExtractedParam>,
    pub similarity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedParam {
    pub value: String,
    pub mask_name: String,
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

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

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

fn get_drain3_template(log_line: &str) -> Option<(String, Vec<ExtractedParam>)> {
    let output = Command::new("python3")
        .args([
            "-c",
            &format!(
                r#"import json; import sys; sys.path.insert(0, '/home/sagnik/Zerch'); from drain3_wrapper import ZerchDrain3; d = ZerchDrain3(); r = d.process_log('{}'); print(json.dumps(r))"#,
                log_line.replace("'", "'\\''")
            ),
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let json_str = String::from_utf8_lossy(&output.stdout);
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json_str) {
        let template = parsed.get("template")?.as_str()?.to_string();
        let params: Vec<ExtractedParam> = parsed
            .get("params")?
            .as_array()?
            .iter()
            .filter_map(|p| {
                Some(ExtractedParam {
                    value: p.get("value")?.as_str()?.to_string(),
                    mask_name: p.get("mask_name")?.as_str()?.to_string(),
                })
            })
            .collect();
        return Some((template, params));
    }
    None
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

#[post("/api/upload")]
async fn upload_logs(
    mut payload: Multipart,
    embedder: web::Data<Arc<Mutex<LocalEmbedder>>>,
    store: web::Data<Arc<QdrantStore>>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut logs: Vec<LogEntry> = Vec::new();
    let mut total_indexed: u32 = 0;

    // Wipe the previous collection so only the new file's embeddings remain.
    if let Err(e) = store.clear().await {
        log::error!("Failed to clear Qdrant collection: {}", e);
        return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "message": format!("Failed to clear vector store: {}", e)
        })));
    }

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let field_name = field.name().to_string();

        if field_name == "file" {
            let mut file_content: Vec<u8> = Vec::new();
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                file_content.extend_from_slice(&data);
            }

            let content_str = String::from_utf8_lossy(&file_content);
            let lines: Vec<&str> = content_str.lines().collect();
            log::info!("Processing file with {} lines", lines.len());

            let mut batch: Vec<(Vec<f32>, serde_json::Value)> = Vec::new();
            let mut seen_templates: HashMap<String, Vec<f32>> = HashMap::new();
            const BATCH_SIZE: usize = 64;

            for (idx, line) in lines.iter().enumerate() {
                if line.trim().is_empty() {
                    continue;
                }

                let raw_log = line.to_string();

                let (template, params) = get_drain3_template(&raw_log)
                    .unwrap_or((raw_log.clone(), vec![]));

                let vector = if let Some(cached_vector) = seen_templates.get(&template) {
                    log::debug!("Template already exists: {} - re-using vector", template);
                    cached_vector.clone()
                } else {
                    let vector = {
                        let mut emb = embedder.lock().await;
                        match emb.embed(&template) {
                            Ok(v) => v,
                            Err(e) => {
                                log::error!("Failed to embed template: {}", e);
                                vec![0.5; 384]
                            }
                        }
                    };
                    seen_templates.insert(template.clone(), vector.clone());
                    vector
                };

                let entry_id = format!("log-{}", Uuid::new_v4());

                let payload = serde_json::json!({
                    "raw_log": raw_log,
                    "template": template,
                    "params": params,
                    "cluster_id": entry_id
                });

                batch.push((vector.clone(), payload.clone()));

                if batch.len() >= BATCH_SIZE {
                    if let Err(e) = store.append_templates_batch(&batch).await {
                        log::error!("Failed to batch upsert: {}", e);
                    }
                    batch.clear();
                }

                let log_entry = LogEntry {
                    id: entry_id,
                    ts: extract_timestamp(&raw_log),
                    level: extract_log_level(&raw_log),
                    service: extract_service(&raw_log),
                    msg: raw_log,
                    template,
                    params,
                    similarity: 0.75,
                };

                log::info!("[{}] Indexed: {}", total_indexed + 1, log_entry.msg);
                logs.push(log_entry);
                total_indexed += 1;
            }

            if !batch.is_empty() {
                if let Err(e) = store.append_templates_batch(&batch).await {
                    log::error!("Failed to batch upsert remaining: {}", e);
                }
            }
        }
    }

    let response = UploadResponse {
        success: true,
        logs,
        count: total_indexed as usize,
        message: format!("Successfully indexed {} logs into Qdrant", total_indexed),
    };

    log::info!("Upload complete: {} logs indexed into Qdrant", total_indexed);
    Ok(HttpResponse::Ok().json(response))
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "engine": "Running (Rust + Qdrant)"
    }))
}

#[get("/api/search")]
async fn search(
    query: web::Query<SearchQuery>,
    embedder: web::Data<Arc<Mutex<LocalEmbedder>>>,
    store: web::Data<Arc<QdrantStore>>,
) -> Result<HttpResponse, actix_web::Error> {
    let search_query = query.q.trim().to_string();

    if search_query.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "message": "Search query cannot be empty"
        })));
    }

    let limit = query.limit.unwrap_or(5) as u64;

    // Auto-reindex if no data
    let count = store.count().await.map_err(|e| {
        log::error!("Count error: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Count error: {}", e))
    })?;
    if count == 0 {
        log::info!("No data found, triggering auto-reindex from test.log...");
        let test_log = std::path::Path::new("test.log");
        if test_log.exists() {
            let file = std::fs::File::open(test_log)?;
            let reader = std::io::BufReader::new(file);
            use std::io::BufRead;
            
            store.clear().await.map_err(|e| {
                log::error!("Clear error: {}", e);
                actix_web::error::ErrorInternalServerError(format!("Clear error: {}", e))
            })?;

            let mut embedder = embedder.lock().await;
            let mut batch: Vec<(Vec<f32>, serde_json::Value)> = Vec::new();
            const BATCH_SIZE: usize = 64;

            for line in reader.lines() {
                if let Ok(raw_log) = line {
                    if raw_log.is_empty() {
                        continue;
                    }
                    let vector = match embedder.embed(&raw_log) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("Embed error: {}", e);
                            continue;
                        }
                    };
                    let (template, params) = get_drain3_template(&raw_log).unwrap_or((raw_log.clone(), vec![]));
                    let payload = serde_json::json!({
                        "raw_log": raw_log,
                        "template": template,
                        "params": params
                    });
                    batch.push((vector, payload));
                    if batch.len() >= BATCH_SIZE {
                        if let Err(e) = store.append_templates_batch(&batch).await {
                            log::error!("Batch upsert error: {}", e);
                        }
                        batch.clear();
                    }
                }
            }
            if !batch.is_empty() {
                if let Err(e) = store.append_templates_batch(&batch).await {
                    log::error!("Final batch upsert error: {}", e);
                }
            }
        } else {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "message": "No data found and test.log not found - please upload a log file"
            })));
        }
    }

    // Embed the raw query directly (matching CLI behavior - not using template)
    let query_vector = {
        let mut emb = embedder.lock().await;
        emb.embed(&search_query).map_err(|e| {
            log::error!("Embed error: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Embed error: {}", e))
        })?
    };

    // Search Qdrant with more results for metadata filtering
    let fetch_limit = std::cmp::max(limit * 3, 30);
    let raw_results = store.search_with_metadata(query_vector, fetch_limit).await.map_err(|e| {
        log::error!("Qdrant search error: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Search error: {}", e))
    })?;

    // Extract search params from query (IP addresses, numbers, etc.)
    let search_terms: Vec<&str> = search_query.split_whitespace().collect();
    let query_ips: Vec<&str> = search_terms.iter()
        .filter(|s| s.contains('.') && s.chars().filter(|c| *c == '.').count() == 3)
        .map(|s| *s)
        .collect();
    let query_numbers: Vec<&str> = search_terms.iter()
        .filter(|s| s.chars().all(|c| c.is_ascii_digit() || c == '.'))
        .map(|s| *s)
        .collect();

    // Filter and re-rank by metadata match
    let mut scored_results: Vec<(f32, String, bool)> = raw_results
        .into_iter()
        .map(|(score, text, metadata)| {
            // Check if metadata contains search values
            let metadata_str = serde_json::to_string(&metadata).unwrap_or_default();
            let mut is_exact_match = false;
            
            // Check IP matches
            for ip in &query_ips {
                if metadata_str.contains(ip) {
                    is_exact_match = true;
                    break;
                }
            }
            // Check number matches  
            if !is_exact_match {
                for num in &query_numbers {
                    if metadata_str.contains(num) {
                        is_exact_match = true;
                        break;
                    }
                }
            }
            
            // Exact match gets score boost
            let boosted_score = if is_exact_match { score * 1.0 + 0.1 } else { score };
            (boosted_score, text, is_exact_match)
        })
        .collect();

    // Sort by boosted score
    scored_results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    // Take top results
    let search_results: Vec<SearchResult> = scored_results
        .into_iter()
        .take(limit as usize)
        .enumerate()
        .map(|(i, (score, text, _))| SearchResult {
            id: format!("result-{}", i),
            score,
            text,
        })
        .collect();

    let count = search_results.len();
    let response = SearchResponse {
        success: true,
        query: search_query,
        results: search_results,
        count,
        message: format!("Found {} matching logs", count),
    };

    log::info!("Search complete: {} results from Qdrant", count);
    Ok(HttpResponse::Ok().json(response))
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

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

    let qdrant_url = std::env::var("QDRANT_URL")
        .unwrap_or_else(|_| "http://localhost:6334".to_string());

    log::info!("Connecting to Qdrant at {}...", qdrant_url);
    let store = match QdrantStore::new(&qdrant_url).await {
        Ok(s) => {
            log::info!("Connected to Qdrant");
            s
        }
        Err(e) => {
            log::error!("Failed to connect to Qdrant: {}", e);
            panic!("Cannot start server without Qdrant");
        }
    };

    // Ensure the collection is ready
    if let Err(e) = store.init_collection().await {
        log::error!("Failed to initialize Qdrant collection: {}", e);
        panic!("Cannot initialize Qdrant collection");
    }

    let store = Arc::new(store);
    let client = Client::new();

    let host = std::env::var("ZERCH_API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = std::env::var("ZERCH_API_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let bind_addr = format!("{}:{}", host, port);
    log::info!("Starting Zerch API server on http://{}", bind_addr);

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
    .bind(&bind_addr)?
    .run()
    .await
}
