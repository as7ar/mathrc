use std::ops::{Add, AddAssign, Index, IndexMut, Mul};

use crate::err::MatrixError;
use num_traits::Float;

/// A generic mathematical matrix.
///
/// Stores elements in row-major order.
///
/// # Type Parameters
///
/// * `T` - Numeric type implementing [`Float`].
///
/// # Examples
///
/// ```rust
/// use mathrc::Matrix;
///
/// let matrix = Matrix::new(
///     vec![1.0, 2.0, 3.0, 4.0],
///     2,
///     2,
/// ).unwrap();
///
/// assert_eq!(matrix.rows(), 2);
/// assert_eq!(matrix.cols(), 2);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T: Float> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Float + AddAssign> Matrix<T> {
    /// Creates a new matrix.
    ///
    /// # Arguments
    ///
    /// * `data` - Matrix elements in row-major order.
    /// * `rows` - Number of rows.
    /// * `cols` - Number of columns.
    ///
    /// # Errors
    ///
    /// Returns [`MatrixError::InvalidSize`] if
    /// `data.len() != rows * cols`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Matrix;
    ///
    /// let matrix = Matrix::new(
    ///     vec![1.0, 2.0, 3.0, 4.0],
    ///     2,
    ///     2,
    /// ).unwrap();
    /// ```
    pub fn new(data: Vec<T>, rows: usize, cols: usize) -> Result<Self, MatrixError> {
        let expected = rows * cols;
        if data.len() != expected {
            return Err(MatrixError::InvalidSize {
                expected,
                got: data.len(),
            });
        }
        Ok(Self { data, rows, cols })
    }

    /// Creates an identity matrix of size `n × n`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Matrix;
    ///
    /// let matrix = Matrix::<f64>::identity(3);
    /// ```
    pub fn identity(n: usize) -> Self {
        let mut data = vec![T::zero(); n * n];
        for i in 0..n {
            data[i * n + i] = T::one();
        }
        Self {
            data,
            rows: n,
            cols: n,
        }
    }

    /// Returns the number of rows.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns the value at the given position.
    ///
    /// # Arguments
    ///
    /// * `r` - Row index.
    /// * `c` - Column index.
    ///
    /// Returns `None` if the index is out of bounds.
    pub fn get(&self, r: usize, c: usize) -> Option<T> {
        if r >= self.rows || c >= self.cols {
            return None;
        }
        Some(self.data[r * self.cols + c])
    }

    /// Sets the value at the given position.
    ///
    /// # Arguments
    ///
    /// * `r` - Row index.
    /// * `c` - Column index.
    /// * `value` - New value.
    pub fn set(&mut self, r: usize, c: usize, value: T) {
        self.data[r * self.cols + c] = value;
    }

    /// Returns the transpose of the matrix.
    ///
    /// Rows become columns and columns become rows.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Matrix;
    ///
    /// let matrix = Matrix::new(
    ///     vec![1.0, 2.0, 3.0, 4.0],
    ///     2,
    ///     2,
    /// ).unwrap();
    ///
    /// let transposed = matrix.transpose();
    /// ```
    pub fn transpose(&self) -> Self {
        let mut data = vec![T::zero(); self.rows * self.cols];
        for r in 0..self.rows {
            for c in 0..self.cols {
                data[c * self.rows + r] = self[(r, c)];
            }
        }
        Self {
            data,
            rows: self.cols,
            cols: self.rows,
        }
    }
}

impl<T: Float> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    /// Returns an immutable reference to a matrix element.
    fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
        &self.data[r * self.cols + c]
    }
}

impl<T: Float> IndexMut<(usize, usize)> for Matrix<T> {
    /// Returns a mutable reference to a matrix element.
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut Self::Output {
        &mut self.data[r * self.cols + c]
    }
}

impl<T: Float + AddAssign> Add for Matrix<T> {
    type Output = Result<Self, MatrixError>;

    /// Performs matrix addition.
    ///
    /// # Errors
    ///
    /// Returns [`MatrixError::DimensionMismatch`]
    /// if the matrix dimensions differ.
    fn add(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            return Err(MatrixError::DimensionMismatch {
                lhs: (self.rows, self.cols),
                rhs: (rhs.rows, rhs.cols),
            });
        }
        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| *a + *b)
            .collect();
        Ok(Self {
            data,
            rows: self.rows,
            cols: self.cols,
        })
    }
}

impl<T: Float + AddAssign> Mul for Matrix<T> {
    type Output = Result<Self, MatrixError>;

    /// Performs matrix multiplication.
    ///
    /// # Errors
    ///
    /// Returns [`MatrixError::DimensionMismatch`]
    /// if the matrices cannot be multiplied.
    fn mul(self, rhs: Self) -> Self::Output {
        if self.cols != rhs.rows {
            return Err(MatrixError::DimensionMismatch {
                lhs: (self.rows, self.cols),
                rhs: (rhs.rows, rhs.cols),
            });
        }
        let mut data = vec![T::zero(); self.rows * rhs.cols];
        for r in 0..self.rows {
            for c in 0..rhs.cols {
                let mut sum = T::zero();
                for k in 0..self.cols {
                    sum += self[(r, k)] * rhs[(k, c)];
                }
                data[r * rhs.cols + c] = sum;
            }
        }
        Ok(Self {
            data,
            rows: self.rows,
            cols: rhs.cols,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_element() {
        let m = Matrix::new(
            vec![
                0f64, 1f64, 0f64, 1f64, 0f64, 1f64, 1f64, 0f64, 1f64, 0f64, 1f64, 0f64,
            ],
            3,
            4,
        )
        .unwrap();
        assert_eq!(m.get(1, 2), Some(1f64));
    }

    #[test]
    fn identity_matrix() {
        let m = Matrix::<f64>::identity(2);
        assert_eq!(m.get(0, 0), Some(1f64));
        assert_eq!(m.get(0, 1), Some(0f64));
        assert_eq!(m.get(1, 1), Some(1f64));
    }

    #[test]
    fn add_matrices() {
        let a = Matrix::new(vec![1f64, 2f64, 3f64, 4f64], 2, 2).unwrap();
        let b = Matrix::new(vec![5f64, 6f64, 7f64, 8f64], 2, 2).unwrap();
        let c = (a + b).unwrap();
        assert_eq!(c.get(0, 0), Some(6f64));
        assert_eq!(c.get(1, 1), Some(12f64));
    }

    #[test]
    fn mul_matrices() {
        let a = Matrix::new(vec![1f64, 2f64, 3f64, 4f64], 2, 2).unwrap();
        let b = Matrix::new(vec![5f64, 6f64, 7f64, 8f64], 2, 2).unwrap();
        let c = (a * b).unwrap();
        assert_eq!(c.get(0, 0), Some(19f64));
        assert_eq!(c.get(1, 1), Some(50f64));
    }
}
