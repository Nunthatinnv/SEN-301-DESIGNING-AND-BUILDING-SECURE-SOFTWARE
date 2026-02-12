// src/lab_b.rs — Reference Solution (Instructor)
//
// Fixes demonstrated float bugs by:
//   - Avoiding u64->f64 conversion for quota logic (integer math + checked ops)
//   - Rejecting NaN / infinity explicitly
//   - Making float threshold checks robust (explicit epsilon + finite checks)
//
// Note: `run()` is unchanged (as required).

#[derive(Debug)]
pub enum LabError {
    InvalidInput,
    NaNOrInfinity,
    PrecisionLoss,
}

pub type Result<T> = std::result::Result<T, LabError>;

pub fn run() {
    println!("=== Lab B: Floating-Point Safety (buggy demo) ===");

    // Demo 1: Precision loss when casting large integers to f64
    let used = 9_007_199_254_740_993u64;   // 2^53 + 1
    let limit = 9_007_199_254_740_992u64;  // 2^53

    let allowed = quota_check_buggy(used, limit)
        .expect("BUG: precision loss not handled");
    println!("Quota check says allowed? {allowed}");

    // Demo 2: NaN / infinity propagation
    let allowed2 = ratio_check_buggy(0.0, 0.0)
        .expect("BUG: NaN / infinity not handled");
    println!("Ratio check with 0/0 says allowed? {allowed2}");

    // Demo 3: Threshold comparison instability
    let a = 0.1f64;
    let b = 0.2f64;
    let c = 0.3f64;

    let ok = threshold_check_buggy(a, b, c)
        .expect("BUG: float comparison instability not handled");
    println!("Threshold check (0.1 + 0.2 <= 0.3) says ok? {ok}");
}

/// FIX #1: Quota logic without floats
///
/// Original intent: "allow if used/limit < 0.90".
/// Integer rewrite:
///   used/limit < 9/10   <=>   used * 10 < limit * 9
///
/// This avoids f64 precision loss entirely.
/// We also:
///   - validate limit != 0
///   - use checked_mul to avoid overflow
pub fn quota_check_buggy(used: u64, limit: u64) -> Result<bool> {
    if limit == 0 {
        return Err(LabError::InvalidInput);
    }

    let lhs = used.checked_mul(10).ok_or(LabError::InvalidInput)?;
    let rhs = limit.checked_mul(9).ok_or(LabError::InvalidInput)?;
    Ok(lhs < rhs)
}

/// FIX #2: Reject NaN / infinity explicitly
///
/// We define safe behavior:
///   - If denom == 0.0 (or r is non-finite), return Err.
///   - Otherwise, proceed with the comparison.
///
/// Note: You could also avoid floats here entirely if the inputs were integers.
/// This lab keeps it float-focused but safe.
pub fn ratio_check_buggy(numer: f64, denom: f64) -> Result<bool> {
    if !numer.is_finite() || !denom.is_finite() {
        return Err(LabError::NaNOrInfinity);
    }
    if denom == 0.0 {
        return Err(LabError::InvalidInput);
    }

    let r = numer / denom;
    if !r.is_finite() {
        return Err(LabError::NaNOrInfinity);
    }

    Ok(r < 0.9)
}

/// FIX #3: Robust threshold check with explicit epsilon
///
/// Original buggy behavior:
///   a + b <= c is unstable for values like 0.1, 0.2, 0.3.
///
/// We choose a conservative comparison:
///   treat values within epsilon as "equal enough".
///
/// Security note:
///   - If this were money/quota logic, we'd prefer fixed-point integers.
///   - Here we explicitly document epsilon and require finite inputs.
pub fn threshold_check_buggy(a: f64, b: f64, c: f64) -> Result<bool> {
    if !a.is_finite() || !b.is_finite() || !c.is_finite() {
        return Err(LabError::NaNOrInfinity);
    }

    let sum = a + b;
    if !sum.is_finite() {
        return Err(LabError::NaNOrInfinity);
    }

    // A small tolerance appropriate for simple demos (NOT a universal constant).
    // We also scale epsilon with magnitude to avoid issues at larger values.
    let abs_eps = 1e-12;
    let rel_eps = 1e-12 * c.abs().max(1.0);
    let eps = abs_eps.max(rel_eps);

    Ok(sum <= c + eps)
}
