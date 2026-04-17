use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use zerch_embed::LocalEmbedder;
use zerch_storage::QdrantStore;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let qdrant_url =
        std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6334".to_string());

    println!("Connecting to Qdrant at {}...", qdrant_url);
    let store = QdrantStore::new(&qdrant_url).await?;
    store.init_collection().await?;

    println!("Loading model... this might take a moment.");
    let mut embedder = LocalEmbedder::load()?;

    if args.len() < 2 {
        eprintln!("Usage: zerch-engine --search <query>");
        eprintln!("       zerch-engine <log-file>");
        return Ok(());
    }

    if args[1] == "--search" {
        // ── SEARCH ─────────────────────────────────────────────────────────
        if args.len() < 3 {
            eprintln!("Error: Please provide a search query.");
            return Ok(());
        }

        // Check if data exists, if not re-index
        let count = store.count().await?;
        if count == 0 {
            println!("No data found, re-indexing from test.log...");
            index_file(&store, &mut embedder, "test.log").await?;
        }

        let query = &args[2];
        println!("Searching for: \"{}\"", query);

        let start_time = Instant::now();
        let query_vector = embedder.embed(query)?;
        println!("Query embedded in {:?}", start_time.elapsed());

        // Extract IPs and numbers from query for metadata filtering
        let search_terms: Vec<&str> = query.split_whitespace().collect();
        let query_ips: Vec<&str> = search_terms.iter()
            .filter(|s| s.contains('.') && s.chars().filter(|c| *c == '.').count() == 3)
            .map(|s| *s)
            .collect();
        let query_numbers: Vec<&str> = search_terms.iter()
            .filter(|s| s.chars().all(|c| c.is_ascii_digit() || c == '.'))
            .map(|s| *s)
            .collect();

        // Search with more results for metadata filtering
        let raw_results = store.search_with_metadata(query_vector, 30).await?;

        // Filter and re-rank by metadata match - check raw text for exact matches
        let mut scored_results: Vec<(f32, String)> = raw_results
            .into_iter()
            .map(|(score, text, _metadata)| {
                let mut is_exact_match = false;
                
                // Check if search values are in the result text
                for ip in &query_ips {
                    if text.contains(ip) {
                        is_exact_match = true;
                        break;
                    }
                }
                if !is_exact_match {
                    for num in &query_numbers {
                        if text.contains(num) {
                            is_exact_match = true;
                            break;
                        }
                    }
                }
                
                // Exact match gets score boost
                let boosted_score = if is_exact_match { score * 1.0 + 0.1 } else { score };
                (boosted_score, text)
            })
            .collect();

        scored_results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        let results: Vec<(f32, String)> = scored_results.into_iter().take(5).collect();

        println!("\nTop 5 Results:");
        println!("--------------------------------------------------");
        for (i, (score, text)) in results.iter().enumerate() {
            println!("[{}] Score: {:.4} | {}", i + 1, score, text);
        }
    } else {
        // ── INDEXING ────────────────────────────────────────────────────────
        let file_path = &args[1];
        index_file(&store, &mut embedder, file_path).await?;
    }

    Ok(())
}

async fn index_file(store: &QdrantStore, embedder: &mut LocalEmbedder, file_path: &str) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    println!("Clearing existing vectors...");
    store.clear().await?;

    let start_time = Instant::now();
    let mut count: u32 = 0;
    let mut batch: Vec<(Vec<f32>, String)> = Vec::new();
    const BATCH_SIZE: usize = 64;

    println!("Indexing logs from: {}...", file_path);

    for line in reader.lines() {
        let log = line?;
        if log.is_empty() {
            continue;
        }

        let vector = embedder.embed(&log)?;
        batch.push((vector, log.clone()));

        count += 1;
        println!("[{}] Embedded: {}", count, log);

        if batch.len() >= BATCH_SIZE {
            store.append_vectors_batch(&batch).await?;
            batch.clear();
        }
    }

    if !batch.is_empty() {
        store.append_vectors_batch(&batch).await?;
    }

    let total_duration = start_time.elapsed();
    let avg_per_log = if count > 0 {
        total_duration / count
    } else {
        total_duration
    };

    println!("\nAll logs successfully stored in Qdrant!");
    println!("--------------------------------------------------");
    println!("Total logs indexed: {}", count);
    println!("Total Indexing Time: {:?}", total_duration);
    println!("Average time per log: {:?}", avg_per_log);
    println!("--------------------------------------------------");

    Ok(())
}
