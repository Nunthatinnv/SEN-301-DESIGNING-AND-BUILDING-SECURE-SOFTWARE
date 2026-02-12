// Lab A — Integer Safety
//
// Goals:
//   - Observe how integer bugs manifest in Rust (panic vs silent wrap in release).
//   - Fix each bug using correct Rust patterns (checked arithmetic, TryFrom, validation).
//
// Running:
//   cargo run
//   cargo run --release   (compare behavior)
//
// =======================================
// TODO 
// =======================================
// 1) Fix `alloc_records_buggy`:
//      - Prevent overflow in `count * record_size`.
//      - Reject unreasonably large allocations (pick a MAX_BYTES cap).
//      - Avoid `as` casts for untrusted numeric conversions.
//      - Must NOT panic on malformed inputs; return Result instead.
//
// 2) Fix `read_at_offset_buggy`:
//      - Prevent signedness bugs: negative offsets must be rejected.
//      - Prevent out-of-bounds indexing.
//      - Must NOT panic; return Result.
//
// 3) Fix `parse_port_buggy`:
//      - Prevent truncation: do NOT use `as` to squeeze u64 into u16.
//      - Reject values outside [1, 65535].
//      - Must return Result.
//
// 4) Fix `avg_chunk_size_buggy`:
//      - Prevent divide-by-zero.
//      - Must return Result.
//
// =======================================
//
// Hints (high level):
//   - Use checked_mul / checked_add.
//   - Consider capping allocations with a MAX_BYTES constant.
//   - Use usize::try_from(...) and validate ranges.
//   - Handle offset < 0 explicitly before conversion.
//   - Use u16::try_from(...) and validate port range.
//   - If divisor == 0, return Err(DivideByZero). 
//
// =======================================

use std::convert::TryFrom;

#[derive(Debug)]
enum LabError {
    Overflow,
    TooLarge,
    NegativeOffset,
    OutOfBounds,
    InvalidPort,
    DivideByZero,
}

type Result<T> = std::result::Result<T, LabError>;

/// BUG #1: overflow in size calculation + unchecked cast to usize
///
/// Scenario: we parse a "packet header" containing `count` and `record_size`.
/// We then allocate a buffer of `count * record_size` bytes.
///
/// In debug builds, the multiplication may panic on overflow.
/// In release builds, it may wrap, resulting in a too-small allocation.
///
/// Your task: make this safe and return Result<Vec<u8>>.
fn alloc_records_buggy(count: u32, record_size: u32) -> Vec<u8> {
    // BUG: overflow is possible; in release this wraps.
    let total_bytes_u32 = count * record_size;

    // BUG: unchecked cast; also hides truncation when going to usize on some platforms.
    let total = total_bytes_u32 as usize;

    vec![0u8; total]
}

/// BUG #2: signedness / cast bug + out-of-bounds
///
/// Scenario: an offset arrives from the network as i32 (signed).
/// Developer casts it to usize and indexes into the buffer.
///
/// If offset is negative, casting to usize produces a huge number.
/// Indexing panics.
///
/// Your task: return Result<u8>.
fn read_at_offset_buggy(buf: &[u8], offset: i32) -> u8 {
    let idx = offset as usize; // BUG: negative -> huge
    buf[idx] // BUG: may panic
}

/// BUG #3: truncation bug
///
/// Scenario: user supplies a "port" field as u64.
/// Developer stores it as u16 using `as`, silently truncating.
///
/// Example: 70000 becomes 4464 (70000 mod 65536).
///
/// Your task: enforce range [1, 65535] and return Result<u16>.
fn parse_port_buggy(port_from_user: u64) -> u16 {
    port_from_user as u16 // BUG: silent truncation
}

/// BUG #4: divide-by-zero
///
/// Scenario: compute average bytes per chunk.
/// If chunks == 0, this panics.
///
/// Your task: return Result<u64>.
fn avg_chunk_size_buggy(total_bytes: u64, chunks: u64) -> u64 {
    total_bytes / chunks // BUG: divide by zero
}

pub fn run() {
    println!("=== Lab A: Integer Safety (buggy demo) ===");

    // ------------------------------------------------------------
    // Demo 1: Overflow in allocation size
    // ------------------------------------------------------------
    // Chosen values overflow u32 when multiplied:
    //   100_000 * 100_000 = 10_000_000_000 > u32::MAX
    //
    // Debug: may panic during multiplication in alloc_records_buggy.
    // Release: wraps and allocates a much smaller buffer than intended.
    let count = 100_000u32;
    let record_size = 100_000u32;

    let buf = alloc_records_buggy(count, record_size);
    println!("Allocated buffer length: {}", buf.len());

    // ------------------------------------------------------------
    // Demo 2: Signedness cast bug
    // ------------------------------------------------------------
    // Negative offset becomes huge when cast to usize, causing panic.
    let data = vec![1u8, 2, 3, 4, 5];
    let offset = -1i32;

    let _x = read_at_offset_buggy(&data, offset);
    println!("Read byte: {}", _x);

    // ------------------------------------------------------------
    // Demo 3: Truncation bug
    // ------------------------------------------------------------
    // 70000 is not a valid TCP/UDP port, but truncation produces 4464.
    let p = parse_port_buggy(70_000);
    println!("Parsed port: {}", p);

    // ------------------------------------------------------------
    // Demo 4: Divide by zero
    // ------------------------------------------------------------
    let avg = avg_chunk_size_buggy(1024, 0);
    println!("Average chunk size: {}", avg);
}
