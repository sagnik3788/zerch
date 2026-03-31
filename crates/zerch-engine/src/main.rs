use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use zerch_core::cosine_similarity;
use zerch_embed::LocalEmbedder;
use zerch_storage::VectorStore;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    println!("Loading model... this might take a moment.");
    let mut embedder = LocalEmbedder::load()?;
    let store = VectorStore::new("zerch_data.bin");

    if args[1] == "--search" {
        // --- SEARCH  ---
        if args.len() < 3 {
            println!("Error: Please provide a search query.");
            return Ok(());
        }

        let query = &args[2];
        println!("Searching for: \"{}\"", query);

        let start_time = Instant::now();

        // Embed the query
        let _query_vector = embedder.embed(query)?;
        println!("Query embedded in {:?}", start_time.elapsed());

        let mut file = File::open("zerch_data.bin")?;

        let mut top_matches: Vec<(f32, String)> = Vec::new();

        // loop over each vector+text and return the top matches
        loop {
            // read the vec len
            let mut len_buf = [0u8; 4];
            if file.read_exact(&mut len_buf).is_err() {
                break;
            }
            let vec_len = u32::from_le_bytes(len_buf) as usize;

            //read vec data
            let mut vec_bytes = vec![0u8; vec_len * 4];
            file.read_exact(&mut vec_bytes)?;

            // Convert
            let mut log_vector = Vec::with_capacity(vec_len);
            for chunk in vec_bytes.chunks_exact(4) {
                let bytes: [u8; 4] = chunk.try_into().unwrap();
                log_vector.push(f32::from_le_bytes(bytes));
            }

            let score = cosine_similarity(&_query_vector, &log_vector).score;

            //text len
            file.read_exact(&mut len_buf)?;
            let text_len = u32::from_le_bytes(len_buf) as usize;

            // Read Text Bytes
            let mut text_bytes = vec![0u8; text_len];
            file.read_exact(&mut text_bytes)?;
            let text = String::from_utf8_lossy(&text_bytes).into_owned();

            // track of the top results
            top_matches.push((score, text));
            top_matches.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            top_matches.truncate(5);
        }

        println!("\nTop 5 Results:");
        println!("--------------------------------------------------");
        for (i, (score, text)) in top_matches.iter().enumerate() {
            println!("[{}] Score: {:.4} | {}", i + 1, score, text);
        }
    } else {
        // --- INDEXING  ---
        let file_path = &args[1];
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let start_time = Instant::now();
        let mut count: u32 = 0;

        println!("Indexing logs from: {}...", file_path);

        for line in reader.lines() {
            let log = line?;
            if log.is_empty() {
                continue;
            }

            // Generate the vector and store it along with the original text
            let vector = embedder.embed(&log)?;
            store.append_vector(&vector, &log)?;

            count += 1;
            println!("[{}] Indexed: {}", count, log);
        }

        let total_duration = start_time.elapsed();
        let avg_per_log = if count > 0 {
            total_duration / count
        } else {
            total_duration
        };

        println!("\nAll logs have been successfully stored in zerch_data.bin!");
        println!("--------------------------------------------------");
        println!("Total logs indexed: {}", count);
        println!("Total Indexing Time: {:?}", total_duration);
        println!("Average time per log: {:?}", avg_per_log);
        println!("--------------------------------------------------");
    }

    Ok(())
}
