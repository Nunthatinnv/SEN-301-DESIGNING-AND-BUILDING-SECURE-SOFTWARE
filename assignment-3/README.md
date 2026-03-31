# Assignment 3 - Numeric Safety and Error Handling in Rust

## 1. Overview
This assignment uses a toy version of a neural-network-style classifier. The program takes an input matrix, multiplies it by a weight matrix, reduces the result into scores, converts those scores into probabilities, and then classifies each result against a threshold.

The goal is not to build a realistic machine learning system. The goal is to practice numeric safety and error handling in Rust using a small pipeline that is easy to understand.

The weights and inputs are stored in **Q1.15 fixed-point format** using `i16`. This is a compact representation that reduces storage cost compared with `f32` or `f64`, which is useful in quantized or resource-constrained settings. During computation, values can be expanded into wider integer types such as `i32` so intermediate arithmetic has more room before results are narrowed back down.

Q1.15 means:
- the value is stored in a signed 16-bit integer
- 1 bit is used for the integer/sign side and 15 bits are used for the fractional side
- the stored integer actually represents the real number divided by `2^15`

For example:
- `16384` in Q1.15 represents `0.5` because `0.5 * 2^15 = 16384`
- `8192` in Q1.15 represents about `0.25`
- `-16384` in Q1.15 represents about `-0.5`

Multiplication is slightly different in fixed-point arithmetic:
- two Q1.15 values are multiplied as integers
- the intermediate result is wider than the stored format
- the result must be scaled back down by `2^15`
- after that, the program may need to cast the result back into `i16`

That cast-back step is one of the important numeric-safety points in this assignment.

If you want more background on fixed-point formats, a good starting point is the Wikipedia article on [Q number format](https://en.wikipedia.org/wiki/Q_(number_format)).

The pipeline in this project works like this:

First, `validate_inputs(...)` checks that the matrices and threshold make sense before any math is performed. Next, `matmul_q15(...)` multiplies the input matrix by the weight matrix, and `dot_product_q15(...)` performs the inner arithmetic for each output cell. After that, `row_scores(...)` reduces each activation row into a single integer score. Then `sigmoid_probabilities(...)` converts those integer scores into floating-point probabilities. Finally, `classify(...)` compares each probability against the threshold and produces the final boolean classification.

## 2. TODOs
Look through the assignment files and follow the TODO comments in the code.

The main files to focus on are:
- `src/pipeline.rs`
- `src/error.rs`

### 2.1 Arithmetic
Update the arithmetic so it is explicit and appropriate for the job being done.

In some places, the scaffold currently uses plain arithmetic and unchecked casts. Replace those with safer and clearer numeric behavior. Different parts of the pipeline have different needs:
- some computations need wider intermediate types
- some reductions need a deliberate overflow policy
- some floating-point comparisons need more care near decision boundaries

The point is not to use the same arithmetic strategy everywhere. The point is to choose the arithmetic behavior that makes sense for each stage and make that choice explicit in the code.

There may be multiple correct solutions here.

Important:
If you are unsure which arithmetic option to choose, feel free to add a short comment explaining your choice.

### 2.2 Error Handling
The finished pipeline should not rely on runtime panics for expected failure cases.

Invalid inputs, bad dimensions, narrowing problems, overflow-related failures, and invalid floating-point states should be reported through the pipeline and returned back to `main(...)`. The `main(...)` function should only report the result. It should not be responsible for fixing or hiding errors that should have been handled earlier.

Use the shared error type in:
- `src/error.rs`

All expected failures should be returned as `AppError` values from the pipeline functions instead of causing `panic!`, `assert!`, `unwrap()`, or `expect()` failures at runtime.
