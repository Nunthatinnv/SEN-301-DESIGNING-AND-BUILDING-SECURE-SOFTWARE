// Multi-Threaded Hash Hunt (Range Splitting with Mutex + Atomics)
//
// =======================
// What you are building
// =======================
// You will search for nonces such that:
//
//   hash_hex = SHA256(prefix + ":" + nonce)
//
// is "good enough" under a simple difficulty rule:
//   - the hex string begins with `d` leading '0' characters.
//
// You will do this in parallel using `threads` OS threads.
// Each thread searches a disjoint slice of the nonce range [start, end).
//
// =======================
// Concurrency primitives
// =======================
// We intentionally use three Rust primitives here:
//
// (1) AtomicBool stop
//     - A shared stop signal for early termination.
//     - Threads periodically check stop and exit their loop if stop=true.
//
// (2) AtomicU64 hashes
//     - A shared counter tracking how many hashes have been computed total.
//
// (3) Mutex<Vec<Solution>> results
//     - A shared vector of discovered solutions.
//     - Must be protected by a Mutex because Vec is not safe to mutate concurrently.
//
// =======================
// Your tasks (TODOs)
// =======================
// You will implement ONLY these three TODOs inside the worker loop:
//
// TODO 1: Early stop check
//   - If stop is true, break out of the loop.
//
// TODO 2: Count hashes
//   - Increment `hashes` for every nonce tested.
//
// TODO 3: Publish solutions safely
//   - If a hash meets the difficulty requirement:
//     - Lock `results`
//     - If fewer than k solutions exist, push a new Solution
//     - If you reach k solutions, set stop=true so other threads stop early
//
// =======================
// Correctness requirements
// =======================
// - No unsafe code.
// - All solutions printed must be valid (verify SHA256(prefix:nonce) + difficulty).
// - Output should be deterministic:
//     - We sort solutions by nonce and truncate to k at the end.

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
    // =======================
    // Parameters
    // =======================
    // Feel free to change these when testing.
    // Start with smaller difficulty if your machine is slow:
    //   d=3 or d=4 should produce results quickly.
    let prefix = "cmkl-lab".to_string();
    let d: usize = 7; // leading hex zeros
    let k: usize = 3; // number of solutions to find

    let start: u64 = 0;
    let end: u64 = 100_000_000_000;
    let threads: u64 = 8;

    // =======================
    // Shared state (Arc)
    // =======================
    let stop = Arc::new(AtomicBool::new(false));
    let hashes = Arc::new(AtomicU64::new(0));
    let results = Arc::new(Mutex::new(Vec::<Solution>::new()));

    // =======================
    // Range splitting
    // =======================
    // Each thread i searches:
    //   [start + i*chunk, min(start + (i+1)*chunk, end))
    let n = end - start;
    let chunk = n.div_ceil(threads);

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
                // ---------------------------------------
                // TODO 1: Early stop check
                // If stop is true, exit the loop so we don't waste work.
                // ---------------------------------------
                // TODO: implement

                let hash_hex = sha256_hex(&prefix, nonce);

                // ---------------------------------------
                // TODO 2: Count hashes
                // Increment the shared hash counter for each nonce tested.
                // ---------------------------------------
                // TODO: implement

                if meets_difficulty(&hash_hex, d) {
                    // ---------------------------------------
                    // TODO 3: Publish solution safely
                    // - Lock the results vector
                    // - If results.len() < k, push the new solution
                    // - If results.len() reaches k, set stop=true
                    //
                    // Important:
                    // - Only lock when a solution is found (avoid locking per nonce).
                    // ---------------------------------------
                    // TODO: implement
                }
            }
        }));
    }

    // Wait for workers to finish
    for h in handles {
        h.join().unwrap();
    }

    let elapsed = t0.elapsed();

    // Deterministic output: sort by nonce then truncate to k
    let mut sols = results.lock().unwrap();
    sols.sort_by_key(|s| s.nonce);
    if sols.len() > k {
        sols.truncate(k);
    }

    // Performance stats
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
