// Lab B — Floating-Point Safety 
//
// Your task: Fix the buggy functions so that running
//
//   cargo run -- float
//
// no longer produces incorrect security decisions.
//
// ------------------------------------
// TODO 
// ------------------------------------
// 1) Fix `ratio_check_buggy`:
//      - Explicitly reject NaN and infinity.
//      - Do not allow NaN to silently influence control flow.
//      - Define safe behavior on invalid input (return Err).
//
// 2) Fix `threshold_check_buggy`:
//      - Do NOT rely on direct float equality or unstable thresholds.
//      - Introduce an explicit epsilon
//      - Make the security decision robust and well-defined.
//
// Constraints:
//   - Do not modify `run()`.
//   - Do not use `unsafe`.
//   - Do not introduce new floating-point vulnerabilities.
// ------------------------------------

#[derive(Debug)]
pub enum LabError {
    InvalidInput,
    NaNOrInfinity,
    PrecisionLoss,
}

pub type Result<T> = std::result::Result<T, LabError>;

pub fn run() {
    println!("=== Lab B: Floating-Point Safety (buggy demo) ===");

    // ------------------------------------------------------------
    // Demo 1: NaN / infinity propagation
    // ------------------------------------------------------------
    // 0.0 / 0.0 = NaN
    let allowed = ratio_check_buggy(0.0, 0.0)
        .expect("BUG: NaN / infinity not handled");
    println!("Ratio check with 0/0 says allowed? {allowed}");

    // ------------------------------------------------------------
    // Demo 2: Threshold comparison instability
    // ------------------------------------------------------------
    let a = 0.1f64;
    let b = 0.2f64;
    let c = 0.3f64;

    let ok = threshold_check_buggy(a, b, c)
        .expect("BUG: float comparison instability not handled");
    println!("Threshold check (0.1 + 0.2 <= 0.3) says ok? {ok}");
}

/// BUG #1: NaN / infinity not handled
///
/// Scenario:
///   ratio is used for a security decision.
///   NaN comparisons are always false, which can silently bypass logic.
pub fn ratio_check_buggy(numer: f64, denom: f64) -> Result<bool> {
    let r = numer / denom; // BUG: 0.0/0.0 = NaN
    Ok(r < 0.9)            // BUG: NaN < 0.9 is false
}

/// BUG #2: Float threshold instability
///
/// Scenario:
///   Security / financial threshold logic using floats.
pub fn threshold_check_buggy(a: f64, b: f64, c: f64) -> Result<bool> {
    Ok(a + b <= c) // BUG: 0.1 + 0.2 > 0.3 due to precision
} 
