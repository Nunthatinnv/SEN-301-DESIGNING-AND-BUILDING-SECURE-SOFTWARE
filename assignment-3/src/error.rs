pub type Result<T> = std::result::Result<T, AppError>;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppError {
    EmptyMatrix,
    DimensionMismatch,
    InvalidThreshold,
    ArithmeticOverflow,
    NarrowingConversion,
    NonFiniteFloat,
}

#[allow(dead_code)]
pub fn describe_error(err: &AppError) -> &'static str {
    match err {
        AppError::EmptyMatrix => "matrix inputs must be non-empty and rectangular",
        AppError::DimensionMismatch => "matrix dimensions do not line up for multiplication",
        AppError::InvalidThreshold => {
            "the classification threshold must be finite and within [0.0, 1.0]"
        }
        AppError::ArithmeticOverflow => "intermediate integer arithmetic overflowed",
        AppError::NarrowingConversion => "the Q1.15 output does not fit back into i16",
        AppError::NonFiniteFloat => "probability computation produced NaN or infinity",
    }
}
