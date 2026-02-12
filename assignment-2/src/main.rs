// Assignment: Parallel Proof-of-Work with a Centralized Work Queue
//
// =====================================================================================
// Learning objectives
// =====================================================================================
// By completing this assignment, you should be able to:
//
// 1) Spawn and manage multiple worker threads in Rust.
//    - Create N worker threads.
//    - Store JoinHandle<()> values in a Vec.
//    - Join all workers at the end.
//
// 2) Create and use channels for thread communication (message passing).
//    - Create a channel in main.
//    - Give each worker a Sender<Solution> (clone the sender per worker).
//    - Use the Receiver<Solution> in main to collect results.
//
// 3) Coordinate shared mutable state safely using Arc, Mutex, and atomics.
//    - Global work queue: Arc<Mutex<VecDeque<Work>>>
//    - Cooperative termination: Arc<AtomicBool>
//    - Shared hash counter: Arc<AtomicU64>
//
// =====================================================================================
// Problem: Toy Proof-of-Work (PoW)
// =====================================================================================
// You will search for nonces such that:
//
//   candidate = prefix + ":" + nonce
//   hash_hex  = SHA256(candidate)
//
// A nonce is VALID if `hash_hex` begins with `difficulty` leading '0' hex characters.
// Example: difficulty=4 => hash begins with "0000".
//
// This is a toy model of Proof-of-Work used in blockchains. The purpose of PoW is to
// make it computationally expensive to produce a valid "proof", which helps limit spam
// and makes certain attacks costly. Here we focus on the compute + concurrency aspects.
//
// =====================================================================================
// Concurrency model
// =====================================================================================
// We use a SINGLE global work queue:
//
//   Arc<Mutex<VecDeque<Work>>> work_queue
//
// A Work item represents a small CHUNK of nonces [start, start+len).
//
// Each worker repeatedly:
//   1) Checks the stop flag. If set, exits promptly.
//   2) Takes ONE Work chunk from the global queue (short critical section).
//   3) If the queue is empty, exits.
//   4) Processes the chunk locally (no locks while hashing):
//       - periodically checks stop flag
//       - computes hashes
//       - increments the global hash counter
//       - sends any found solutions back to main via a channel
//
// The main thread acts as the collector:
//   - Receives solutions from workers.
//   - Once K solutions are collected, sets stop=true.
//   - Joins all workers.
//   - Prints deterministic output + performance stats.
//
// =====================================================================================
// Correctness requirements
// =====================================================================================
// - Never hold the queue lock while hashing.
// - Never mutate shared structures without synchronization.
// - Stop should cause workers to exit promptly.
// - Output must be deterministic: sort by nonce, truncate to K.
//
// =====================================================================================

use std::collections::VecDeque;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    mpsc,
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

/// A work item is a chunk of nonces: [start, start+len).
#[derive(Clone, Copy, Debug)]
struct Work {
    start: u64,
    len: u64,
}

impl Work {
    fn end_exclusive(self) -> u64 {
        self.start.saturating_add(self.len)
    }
}

/// Compute SHA-256(prefix + ":" + nonce) and return lowercase hex.
fn sha256_hex(prefix: &str, nonce: u64) -> String {
    let mut h = Sha256::new();
    h.update(prefix.as_bytes());
    h.update(b":");
    h.update(nonce.to_string().as_bytes());
    hex::encode(h.finalize())
}

/// Difficulty check: hash hex begins with `difficulty` leading '0' characters.
fn meets_difficulty(hash_hex: &str, difficulty: usize) -> bool {
    hash_hex
        .as_bytes()
        .iter()
        .take(difficulty)
        .all(|&c| c == b'0')
}

/// Split [start, end) into many small chunks to populate the work queue.
/// Smaller chunk_size => better load balance, more queue contention.
/// Larger chunk_size => less contention, worse early-stop responsiveness.
fn make_initial_work(start: u64, end: u64, chunk_size: u64) -> VecDeque<Work> {
    let mut q = VecDeque::new();
    let mut cur = start;
    while cur < end {
        let remaining = end - cur;
        let len = remaining.min(chunk_size);
        q.push_back(Work { start: cur, len });
        cur += len;
    }
    q
}

/// Worker logic: pull work from the global queue, process, report solutions.
///
/// You must implement the protocol described in the comments.
/// Do not use unsafe code.
fn worker_loop(
    id: usize,
    prefix: String,
    difficulty: usize,
    work_queue: Arc<Mutex<VecDeque<Work>>>,
    sol_tx: mpsc::Sender<Solution>,
    stop: Arc<AtomicBool>,
    hashes: Arc<AtomicU64>,
) {
    // ---------------------------------------------------------------------------------
    // Worker protocol checklist
    // ---------------------------------------------------------------------------------
    // Repeat:
    //
    //   A) If stop flag is set, exit the worker.
    //
    //   B) Take ONE Work chunk from the global queue:
    //        - lock the queue
    //        - pop a chunk (front)
    //        - unlock immediately
    //
    //   C) If no work remains (pop returned None), exit the worker.
    //
    //   D) Process the chunk locally:
    //        - iterate through the nonce range
    //        - occasionally check stop flag, exit early if needed
    //        - compute hash for each nonce
    //        - increment shared hash counter
    //        - if solution found, send it to collector (sol_tx)
    //
    //   E) Loop back to get another chunk.
    //
    // Notes:
    // - The lock around the queue must be held only briefly.
    // - The channel send is how you report results.
    // - Atomics are used only for stop/counter; results are not stored in shared memory.
    // ---------------------------------------------------------------------------------

    // TODO: Implement the worker protocol described above.
    //
    // Remove the panic once implemented.
    panic!("TODO: worker_loop not implemented");
}

fn main() {
    // =================================================================================
    // Parameters (adjustable)
    // =================================================================================
    // If difficulty is too high, you may not find k solutions in [start, end).
    // For early testing, set difficulty to 3 or 4.
    let prefix = "cmkl-pow".to_string();
    let difficulty: usize = 6;
    let k: usize = 10;

    let start: u64 = 0;
    let end: u64 = 50_000_000_000;

    let threads: usize = 8;
    let chunk_size: u64 = 50_000;

    // =================================================================================
    // Shared state
    // =================================================================================
    let stop = Arc::new(AtomicBool::new(false));
    let hashes = Arc::new(AtomicU64::new(0));

    let initial_queue = make_initial_work(start, end, chunk_size);
    let work_queue = Arc::new(Mutex::new(initial_queue));

    let t0 = Instant::now();

    // =================================================================================
    // TODO 1: Channel setup
    // =================================================================================
    // Create a channel for Solution messages.
    //
    // Requirements:
    // - The main thread must own the Receiver<Solution>.
    // - Each worker thread must have a Sender<Solution>.
    // - The Sender must be cloned for each worker.
    //
    // Hint: use std::sync::mpsc (already imported).
    //
    // TODO: create (sol_tx, sol_rx)
    //
    // let (sol_tx, sol_rx) = ...
    panic!("TODO 1: create solution channel");

    // =================================================================================
    // TODO 2: Spawn and manage worker threads
    // =================================================================================
    // Spawn `threads` worker threads and store JoinHandles in a Vec.
    //
    // Requirements:
    // - Each worker gets:
    //     * an id
    //     * prefix.clone()
    //     * difficulty
    //     * Arc clones of work_queue, stop, hashes
    //     * a cloned sender endpoint sol_tx.clone()
    //
    // - You MUST store the JoinHandle so you can join later.
    //
    // - After spawning workers, the main thread should drop its copy of sol_tx.
    //   This ensures the receiver can observe channel closure when workers exit.
    //
    // TODO: create a Vec<JoinHandle<()>> and spawn threads
    //
    // let mut handles = Vec::new();
    // for id in 0..threads { ... }
    // drop(sol_tx);
    panic!("TODO 2: spawn workers");

    // =================================================================================
    // TODO 3: Collector logic (main thread)
    // =================================================================================
    // In the main thread:
    //
    // A) Receive solutions until you have collected k solutions (or channel closes).
    //
    // B) Once you have k solutions:
    //    - Set stop=true (Relaxed is fine here).
    //
    // C) Join all worker threads.
    //
    // D) Use the provided final reporting code below.
    //
    // TODO: implement collector logic and joining.
    //
    let mut solutions: Vec<Solution> = Vec::new();

    panic!("TODO 3: collector logic");
    
    // =================================================================================
    // Final reporting (provided)
    // =================================================================================
    // NOTE: You should reach this point only after:
    // - solutions is collected
    // - stop is set
    // - all workers joined
    //
    // Sort solutions by nonce to make output deterministic
    solutions.sort_by_key(|s| s.nonce);

    // Truncate in case more than k solutions arrived concurrently
    if solutions.len() > k {
        solutions.truncate(k);
    }

    // Performance statistics
    let total_hashes = hashes.load(Ordering::Relaxed);
    let elapsed = t0.elapsed();
    let secs = elapsed.as_secs_f64();
    let hashrate = if secs > 0.0 {
        (total_hashes as f64) / secs
    } else {
        0.0
    };

    println!(
        "prefix={} threads={} difficulty={} target={}",
        prefix, threads, difficulty, k
    );

    println!(
        "chunk_size={} hashes={} time_ms={} hashrate={:.0}/s",
        chunk_size,
        total_hashes,
        elapsed.as_millis(),
        hashrate
    );

    println!("solutions:");
    for s in solutions.iter() {
        println!("nonce={} hash={}", s.nonce, s.hash_hex);
    }

    if solutions.len() < k {
        println!(
            "WARNING: only got {} solutions; lower difficulty or increase end range.",
            solutions.len()
        );
    }
}
