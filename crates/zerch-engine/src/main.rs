use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use zerch_embed::LocalEmbedder;
use zerch_storage::VectorStore;

fn main() -> Result<()> {
    println!("Loading model... this might take a moment.");
    let mut embedder = LocalEmbedder::load()?;

    let store = VectorStore::new("zerch_data.bin");

    let file_path = env::args()
        .nth(1)
        .ok_or(anyhow::anyhow!("No file path provided"))?;
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let start_time = Instant::now();
    let mut count: u32 = 0;

    for line in reader.lines() {
        let log = line?;
        if log.is_empty() {
            continue;
        }
        let vector = embedder.embed(&log)?;
        store.append_vector(&vector)?;
        count += 1;
        println!("[{}] Indexed: {}", count, log);
    }

    let total_duration = start_time.elapsed();
    let avg_per_log = if count > 0 {
        total_duration / count
    } else {
        total_duration
    };

    println!("All logs have been successfully stored in zerch_data.bin!");
    println!("--------------------------------------------------");
    println!("Total logs indexed: {}", count);
    println!("Total Indexing Time: {:?}", total_duration);
    println!("Average time per log: {:?}", avg_per_log);
    println!("--------------------------------------------------");

    Ok(())
}
