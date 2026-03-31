// Quantized Binary Classifier — Student Starter
//
// This starter intentionally contains numeric-safety bugs and incomplete
// validation. Your job is to repair the pipeline code without changing main().

mod error;
mod pipeline;

use crate::error::describe_error;
use crate::pipeline::run_classifier;

fn to_q15(value: f64) -> i16 {
    (value * 32768.0) as i16
}

fn to_q15_matrix(values: &[Vec<f64>]) -> Vec<Vec<i16>> {
    values
        .iter()
        .map(|row| row.iter().map(|value| to_q15(*value)).collect())
        .collect()
}

fn main() {
    println!("=== Quantized Binary Classifier ===");

    // These are readable floating-point demo cases.
    // Feel free to modify them while testing your fixes.
    // The pipeline stores values as Q1.15 fixed-point `i16`, so each matrix is
    // converted before running the classifier.
    let demo_cases = vec![
        (
            "Valid small inference",
            vec![
                vec![0.50, 0.25, -0.25, 0.75],
                vec![-0.50, 0.50, 0.25, -0.25],
                vec![0.10, -0.20, 0.30, 0.40],
                vec![0.90, -0.10, 0.20, -0.30],
            ],
            vec![
                vec![0.40, -0.30, 0.20, 0.10],
                vec![0.25, 0.50, -0.40, 0.30],
                vec![-0.20, 0.10, 0.60, -0.50],
                vec![0.15, -0.35, 0.45, 0.55],
            ],
            0.57,
        ),
        (
            "Dimension mismatch",
            vec![
                vec![0.50, 0.25, -0.25],
                vec![-0.50, 0.50, 0.25],
                vec![0.10, -0.20, 0.30],
                vec![0.90, -0.10, 0.20],
            ],
            vec![
                vec![0.40, -0.30, 0.20, 0.10],
                vec![0.25, 0.50, -0.40, 0.30],
                vec![-0.20, 0.10, 0.60, -0.50],
                vec![0.15, -0.35, 0.45, 0.55],
            ],
            0.57,
        ),
        (
            "Potential narrowing overflow",
            vec![
                vec![0.99, 0.99, 0.99, 0.99],
                vec![0.99, 0.99, 0.99, 0.99],
                vec![0.99, 0.99, 0.99, 0.99],
                vec![0.99, 0.99, 0.99, 0.99],
            ],
            vec![
                vec![0.99, 0.99, 0.99, 0.99],
                vec![0.99, 0.99, 0.99, 0.99],
                vec![0.99, 0.99, 0.99, 0.99],
                vec![0.99, 0.99, 0.99, 0.99],
            ],
            0.50,
        ),
        (
            "Invalid threshold",
            vec![
                vec![0.50, 0.25, -0.25, 0.75],
                vec![-0.50, 0.50, 0.25, -0.25],
                vec![0.10, -0.20, 0.30, 0.40],
                vec![0.90, -0.10, 0.20, -0.30],
            ],
            vec![
                vec![0.40, -0.30, 0.20, 0.10],
                vec![0.25, 0.50, -0.40, 0.30],
                vec![-0.20, 0.10, 0.60, -0.50],
                vec![0.15, -0.35, 0.45, 0.55],
            ],
            f64::NAN,
        ),
    ];

    for (name, input_values, weight_values, threshold) in demo_cases {
        println!("\n--- {} ---", name);

        let inputs = to_q15_matrix(&input_values);
        let weights = to_q15_matrix(&weight_values);

        match run_classifier(&inputs, &weights, threshold) {
            Ok(output) => {
                println!("Activations: {:?}", output.activations);
                println!("Row scores: {:?}", output.scores);
                println!("Probabilities: {:?}", output.probabilities);
                println!("Predictions: {:?}", output.classes);
            }
            Err(err) => {
                println!("Error: {}", describe_error(&err));
            }
        }
    }
}
