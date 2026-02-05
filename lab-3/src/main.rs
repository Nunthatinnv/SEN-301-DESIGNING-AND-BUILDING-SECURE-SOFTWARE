// Multi-Threaded Hash Hunt (Range Splitting)
//
// You will parallelize a toy hash search across multiple threads.
// Hashing and difficulty logic is provided; focus on thread coordination.

use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::Instant;

use sha2::{Digest, Sha256};

#[derive(Clone, Debug)]
struct Solution {
    nonce: u64,
    hash_hex: String,
}

/// Compute SHA-256(prefix + ":" + nonce) and return hex string.
fn sha256_hex(prefix: &str, nonce: u64) -> String {
    let mut h = Sha256::new();
    h.update(prefix.as_bytes());
    h.update(b":");
    h.update(nonce.to_string().as_bytes());
    let digest = h.finalize();
    hex::encode(digest)
}

/// Difficulty check: hash hex begins with `d` leading '0' characters.
fn meets_difficulty(hash_hex: &str, d: usize) -> bool {
    hash_hex.as_bytes().iter().take(d).all(|&c| c == b'0')
}

fn main() {
    // Parameters (you may later wire these to CLI args)
    let prefix = "cmkl-lab".to_string();
    let d: usize = 7; // leading hex zeros
    let k: usize = 3; // number of solutions to find

    let start: u64 = 0;
    let end: u64 = 100_000_000_000;
    let threads: u64 = 8;

    // Shared state
    let stop = Arc::new(AtomicBool::new(false));
    let hashes = Arc::new(AtomicU64::new(0));
    let results = Arc::new(Mutex::new(Vec::<Solution>::new()));

    // Range splitting (provided)
    let n = end - start;
    let chunk = (n + threads - 1) / threads; // ceil div

    let t0 = Instant::now();
    let mut handles = Vec::new();

    for i in 0..threads {
        let lo = start + i * chunk;
        let hi = (lo + chunk).min(end);

        let prefix = prefix.clone();
        let stop = Arc::clone(&stop);
        let hashes = Arc::clone(&hashes);
        let results = Arc::clone(&results);

        handles.push(thread::spawn(move || {
            for nonce in lo..hi {
                // TODO: early stop check

                let hash_hex = sha256_hex(&prefix, nonce);
                hashes.fetch_add(1, Ordering::Relaxed);

                if meets_difficulty(&hash_hex, d) {
                    // TODO: publish solution safely and trigger stop if k reached
                }
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let elapsed = t0.elapsed();

    let mut sols = results.lock().unwrap();
    sols.sort_by_key(|s| s.nonce);
    if sols.len() > k {
        sols.truncate(k);
    }

    let total_hashes = hashes.load(Ordering::Relaxed);
    let secs = elapsed.as_secs_f64();
    let hashrate = if secs > 0.0 { (total_hashes as f64) / secs } else { 0.0 };

    println!(
        "prefix={} threads={} difficulty={} target={}",
        prefix, threads, d, k
    );
    println!(
        "hashes={} time_ms={} hashrate={:.0}/s",
        total_hashes,
        elapsed.as_millis(),
        hashrate
    );

    println!("solutions:");
    for s in sols.iter() {
        println!("nonce={} hash={}", s.nonce, s.hash_hex);
    }
}
