// src/lab_a.rs — Reference Solution (Instructor)
//
// This version fixes all four integer bugs using:
//   - checked arithmetic
//   - fallible conversions (TryFrom)
//   - explicit range / bounds validation
//   - a hard cap on allocations (MAX_BYTES)
//
// Note: `run()` is unchanged (as required).

use std::convert::TryFrom;

#[derive(Debug)]
pub enum LabError {
    Overflow,
    TooLarge,
    NegativeOffset,
    OutOfBounds,
    InvalidPort,
    DivideByZero,
}

pub type Result<T> = std::result::Result<T, LabError>;

const MAX_BYTES: usize = 1_000_000;

pub fn run() {
    println!("=== Lab A: Integer Safety (buggy demo) ===");

    // Demo 1: Overflow in allocation size
    let count = 100_000u32;
    let record_size = 100_000u32;

    let buf = alloc_records_buggy(count, record_size)
        .expect("BUG: overflow / allocation error not handled");
    println!("Allocated buffer length: {}", buf.len());

    // Demo 2: Signedness cast bug
    let data = vec![1u8, 2, 3, 4, 5];
    let offset = -1i32;

    let x = read_at_offset_buggy(&data, offset)
        .expect("BUG: negative offset not handled");
    println!("Read byte at offset {offset}: {x}");

    // Demo 3: Truncation bug
    let p = parse_port_buggy(70_000)
        .expect("BUG: invalid port not handled");
    println!("Parsed port: {p}");

    // Demo 4: Divide-by-zero
    let avg = avg_chunk_size_buggy(1024, 0)
        .expect("BUG: divide by zero not handled");
    println!("Average chunk size: {avg}");
}

/// FIX #1: Safe allocation size computation
///
/// - Prevents overflow using checked_mul.
/// - Converts to usize safely.
/// - Enforces MAX_BYTES cap to prevent DoS allocations.
/// - Returns Result instead of panicking.
pub fn alloc_records_buggy(count: u32, record_size: u32) -> Result<Vec<u8>> {
    // Checked multiplication in the source domain (u32).
    let total_u32 = count.checked_mul(record_size).ok_or(LabError::Overflow)?;

    // Safe conversion to usize (fallible, though u32->usize is infallible on 64-bit,
    // but we keep the pattern explicit for teaching + portability).
    let total_usize = usize::try_from(total_u32).map_err(|_| LabError::Overflow)?;

    if total_usize > MAX_BYTES {
        return Err(LabError::TooLarge);
    }

    Ok(vec![0u8; total_usize])
}

/// FIX #2: Signedness + bounds-safe read
///
/// - Rejects negative offsets before conversion.
/// - Converts i32 -> usize safely.
/// - Bounds-checks before indexing to avoid panic.
pub fn read_at_offset_buggy(buf: &[u8], offset: i32) -> Result<u8> {
    if offset < 0 {
        return Err(LabError::NegativeOffset);
    }

    let idx: usize = usize::try_from(offset).map_err(|_| LabError::OutOfBounds)?;

    buf.get(idx).copied().ok_or(LabError::OutOfBounds)
}

/// FIX #3: No truncation on port parsing
///
/// - Rejects ports outside [1, 65535].
/// - Uses u16::try_from to prevent silent truncation.
pub fn parse_port_buggy(port_from_user: u64) -> Result<u16> {
    if port_from_user == 0 || port_from_user > u16::MAX as u64 {
        return Err(LabError::InvalidPort);
    }
    u16::try_from(port_from_user).map_err(|_| LabError::InvalidPort)
}

/// FIX #4: Divide-by-zero handled explicitly
pub fn avg_chunk_size_buggy(total_bytes: u64, chunks: u64) -> Result<u64> {
    if chunks == 0 {
        return Err(LabError::DivideByZero);
    }
    Ok(total_bytes / chunks)
}
