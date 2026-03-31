use crate::error::Result;

pub const Q15_SCALE: i32 = 1 << 15;

#[derive(Debug, Clone)]
pub struct InferenceOutput {
    pub activations: Vec<Vec<i16>>,
    pub scores: Vec<i32>,
    pub probabilities: Vec<f64>,
    pub classes: Vec<bool>,
}

// What this does:
// Runs the full pipeline from validation to classification.
//
// Pipeline:
// 1. `validate_inputs(...)` checks that the matrices and threshold are valid.
// 2. `matmul_q15(...)` multiplies the input matrix by the weight matrix to
//    produce activations.
// 3. `row_scores(...)` reduces each activation row into a single score.
// 4. `sigmoid_probabilities(...)` converts those scores into probabilities.
// 5. `classify(...)` compares each probability against the threshold.
//
// TODO:
// Keep the same sequence of steps, but handle failures properly and return
// `AppError` instead of causing a runtime panic.
pub fn run_classifier(
    inputs: &[Vec<i16>],
    weights: &[Vec<i16>],
    threshold: f64,
) -> Result<InferenceOutput> {
    validate_inputs(inputs, weights, threshold).expect("input validation failed");
    let activations =
        matmul_q15(inputs, weights).expect("matrix multiplication failed");
    let scores = row_scores(&activations).expect("row scoring failed");
    let probabilities = sigmoid_probabilities(&scores).expect("probability computation failed");
    let classes = classify(&probabilities, threshold).expect("classification failed");

    Ok(InferenceOutput {
        activations,
        scores,
        probabilities,
        classes,
    })
}

// What this does:
// Checks the basic preconditions for the matrix pipeline before computation
// starts.
//
// TODO:
// Keep the same checks, but handle failures properly and return `AppError`
// instead of causing a runtime panic.
pub fn validate_inputs(inputs: &[Vec<i16>], weights: &[Vec<i16>], threshold: f64) -> Result<()> {
    assert!(
        threshold.is_finite() && (0.0..=1.0).contains(&threshold),
        "threshold must be finite and within [0.0, 1.0]"
    );

    assert!(
        !inputs.is_empty() && !weights.is_empty(),
        "input and weight matrices must be non-empty"
    );

    assert!(
        !inputs[0].is_empty() && !weights[0].is_empty(),
        "matrix rows must be non-empty"
    );

    let input_cols = inputs[0].len();
    let weight_cols = weights[0].len();

    assert!(
        inputs.iter().all(|row| row.len() == input_cols),
        "input matrix must be rectangular"
    );
    assert!(
        weights.iter().all(|row| row.len() == weight_cols),
        "weight matrix must be rectangular"
    );
    assert!(
        input_cols == weights.len(),
        "matrix dimensions do not line up for multiplication"
    );

    Ok(())
}

// What this does:
// Computes one quantized dot product.
//
// TODO:
// This function is performance-critical because it performs many repeated
// multiplies in the hot path. Improve the arithmetic and casting so the numeric
// behavior is explicit and safe. In this function, overflow wraparound may be
// an acceptable tradeoff if you make that choice explicit. Any failure in this
// function should be handled properly and returned as `AppError` instead of
// causing a runtime panic.
pub fn dot_product_q15(row: &[i16], col: &[i16]) -> Result<i16> {
    assert_eq!(
        row.len(),
        col.len(),
        "dot product requires equal-length vectors"
    );

    let mut total = 0i32;

    for (&lhs, &rhs) in row.iter().zip(col.iter()) {
        let product = ((lhs as i32) * (rhs as i32)) / Q15_SCALE;
        total += product;
    }

    Ok(total as i16)
}

// What this does:
// Multiplies the input matrix by the weight matrix to produce activations.
//
// TODO:
// Keep the same structure, but handle failures properly and return `AppError`
// instead of causing a runtime panic.
pub fn matmul_q15(inputs: &[Vec<i16>], weights: &[Vec<i16>]) -> Result<Vec<Vec<i16>>> {
    validate_inputs(inputs, weights, 0.5)
        .expect("matrix validation failed");

    let output_cols = weights[0].len();
    let mut activations = vec![vec![0i16; output_cols]; inputs.len()];

    for (row_index, row) in inputs.iter().enumerate() {
        for col_index in 0..output_cols {
            let col = column_at(weights, col_index);
            activations[row_index][col_index] = dot_product_q15(row, &col)
                .expect("dot product failed");
        }
    }

    Ok(activations)
}

// What this does:
// Reduces each activation row into a single integer score.
//
// TODO:
// Keep the same reduction structure, but make the arithmetic explicit and
// return `AppError` instead of relying on runtime panic or unchecked behavior.
pub fn row_scores(activations: &[Vec<i16>]) -> Result<Vec<i32>> {
    assert!(
        !activations.is_empty(),
        "activation matrix must be non-empty"
    );
    assert!(
        !activations[0].is_empty(),
        "activation rows must be non-empty"
    );

    let mut scores = Vec::with_capacity(activations.len());

    for row in activations {
        let mut total = 0i32;
        for &value in row {
            total += value as i32;
        }
        scores.push(total);
    }

    Ok(scores)
}

// What this does:
// Converts integer scores into floating-point probabilities.
//
// TODO:
// Keep the same overall computation, but add proper checks for invalid
// floating-point states and return `AppError` instead of causing a runtime
// panic or silently continuing.
pub fn sigmoid_probabilities(scores: &[i32]) -> Result<Vec<f64>> {
    let mut probabilities = Vec::with_capacity(scores.len());

    for &score in scores {
        let scaled = (score as f64) / (Q15_SCALE as f64);
        let exp_score = scaled.exp();
        let probability = exp_score / (1.0 + exp_score);
        probabilities.push(probability);
    }

    Ok(probabilities)
}

// What this does:
// Compares each probability against the threshold to produce a classification.
//
// TODO:
// Keep the same comparison step, but add proper floating-point validation,
// handle invalid thresholds, and think about values that are extremely close
// to the threshold. A small tolerance may be useful here. Return `AppError`
// instead of causing a runtime panic or silently continuing.
pub fn classify(probs: &[f64], threshold: f64) -> Result<Vec<bool>> {
    let mut classes = Vec::with_capacity(probs.len());
    for &probability in probs {
        classes.push(probability >= threshold);
    }

    Ok(classes)
}

fn column_at(weights: &[Vec<i16>], col_index: usize) -> Vec<i16> {
    let mut col = Vec::with_capacity(weights.len());
    for row in weights {
        col.push(row[col_index]);
    }
    col
}
