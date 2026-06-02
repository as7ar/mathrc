use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum MatrixError {
    InvalidSize { expected: usize, got: usize },
    DimensionMismatch { lhs: (usize, usize), rhs: (usize, usize) },
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatrixError::InvalidSize { expected, got } =>
                write!(f, "invalid matrix size: expected {expected}, got {got}"),
            MatrixError::DimensionMismatch { lhs, rhs } =>
                write!(f, "dimension mismatch: ({} x {}) vs ({} x {})", lhs.0, lhs.1, rhs.0, rhs.1),
        }
    }
}