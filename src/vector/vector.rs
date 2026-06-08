use num_traits::Float;

use crate::err::VectorErr;
use std::fmt;
use std::iter::Sum;
use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

/// Common vector operations.
///
/// This trait provides fundamental vector operations such as
/// dot product, magnitude calculation, and normalization.
///
/// # Type Parameters
///
/// * `T` - Numeric type implementing [`Float`].
pub trait VectorOps<T>
where
    T: Float + Sum<T>,
{
    /// Computes the dot product of two vectors.
    ///
    /// # Errors
    ///
    /// Returns [`VectorErr::DimensionMismatch`] if the vectors
    /// have different dimensions.
    fn dot(&self, other: &Self) -> Result<T, VectorErr>
    where
        Self: Sized;

    /// Returns the magnitude (Euclidean norm) of the vector.
    fn magnitude(&self) -> T;

    /// Returns a normalized version of the vector.
    ///
    /// The resulting vector has a magnitude of `1`.
    ///
    /// # Errors
    ///
    /// Returns [`VectorErr::ZeroVector`] if the vector has zero length.
    fn normalize(&self) -> Result<Self, VectorErr>
    where
        Self: Sized;
}

/// A generic mathematical vector.
///
/// Elements are stored in a dynamic array and may represent
/// vectors of any dimension.
///
/// # Type Parameters
///
/// * `T` - Numeric type implementing [`Float`].
///
/// # Examples
///
/// ```rust
/// use mathrc::Vector;
///
/// let vector = Vector::new(vec![1.0, 2.0, 3.0]);
///
/// assert_eq!(vector[0], 1.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T: Float> {
    vec: Vec<T>,
}

impl<T: Float> Vector<T> {
    /// Creates a new vector.
    ///
    /// # Arguments
    ///
    /// * `vec` - Components of the vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Vector;
    ///
    /// let vector = Vector::new(vec![1.0, 2.0, 3.0]);
    /// ```
    pub fn new(vec: Vec<T>) -> Self {
        Self { vec }
    }

    /// Computes the cross product of two three-dimensional vectors.
    ///
    /// # Errors
    ///
    /// Returns [`VectorErr::DimensionMismatch`] if either vector
    /// does not have exactly three dimensions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Vector;
    ///
    /// let a = Vector::new(vec![1.0, 0.0, 0.0]);
    /// let b = Vector::new(vec![0.0, 1.0, 0.0]);
    ///
    /// let c = a.cross(&b).unwrap();
    ///
    /// assert_eq!(c, Vector::new(vec![0.0, 0.0, 1.0]));
    /// ```
    pub fn cross(&self, other: &Self) -> Result<Self, VectorErr> {
        if self.vec.len() != 3 || other.vec.len() != 3 {
            return Err(VectorErr::DimensionMismatch);
        }
        Ok(Self {
            vec: vec![
                self.vec[1] * other.vec[2] - self.vec[2] * other.vec[1],
                self.vec[2] * other.vec[0] - self.vec[0] * other.vec[2],
                self.vec[0] * other.vec[1] - self.vec[1] * other.vec[0],
            ],
        })
    }
}

impl<T: Float + Sum> VectorOps<T> for Vector<T> {
    /// Computes the dot product of two vectors.
    ///
    /// # Errors
    ///
    /// Returns [`VectorErr::DimensionMismatch`] if the vectors
    /// have different dimensions.
    fn dot(&self, other: &Self) -> Result<T, VectorErr> {
        if self.vec.len() != other.vec.len() {
            return Err(VectorErr::DimensionMismatch);
        }
        Ok(self.vec.iter().zip(&other.vec).map(|(a, b)| *a * *b).sum())
    }

    /// Returns the magnitude (Euclidean norm) of the vector.
    ///
    /// The magnitude is calculated as:
    ///
    /// ```text
    /// √(x₁² + x₂² + ... + xₙ²)
    /// ```
    fn magnitude(&self) -> T {
        self.vec.iter().map(|x| *x * *x).sum::<T>().sqrt()
    }

    /// Returns a unit vector in the same direction.
    ///
    /// # Errors
    ///
    /// Returns [`VectorErr::ZeroVector`] if the vector length is zero.
    fn normalize(&self) -> Result<Self, VectorErr> {
        let len = self.len();
        if len == T::zero() {
            return Err(VectorErr::ZeroVector);
        }
        Ok(Self {
            vec: self.vec.iter().map(|x| *x / len).collect(),
        })
    }
}

impl<T: Float> Add for Vector<T> {
    type Output = Result<Self, VectorErr>;

    /// Performs vector addition.
    ///
    /// # Errors
    ///
    /// Returns [`VectorErr::DimensionMismatch`] if the vectors
    /// have different dimensions.
    fn add(self, other: Self) -> Result<Self, VectorErr> {
        if self.vec.len() != other.vec.len() {
            return Err(VectorErr::DimensionMismatch);
        }
        Ok(Self {
            vec: self
                .vec
                .iter()
                .zip(&other.vec)
                .map(|(a, b)| *a + *b)
                .collect(),
        })
    }
}

impl<T: Float> Sub for Vector<T> {
    type Output = Result<Self, VectorErr>;

    /// Performs vector subtraction.
    ///
    /// # Errors
    ///
    /// Returns [`VectorErr::DimensionMismatch`] if the vectors
    /// have different dimensions.
    fn sub(self, other: Self) -> Result<Self, VectorErr> {
        if self.vec.len() != other.vec.len() {
            return Err(VectorErr::DimensionMismatch);
        }
        Ok(Self {
            vec: self
                .vec
                .iter()
                .zip(&other.vec)
                .map(|(a, b)| *a - *b)
                .collect(),
        })
    }
}

impl<T: Float> Mul<T> for Vector<T> {
    type Output = Self;

    /// Multiplies the vector by a scalar.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Vector;
    ///
    /// let v = Vector::new(vec![1.0, 2.0, 3.0]);
    ///
    /// let result = v * 2.0;
    ///
    /// assert_eq!(result, Vector::new(vec![2.0, 4.0, 6.0]));
    /// ```
    fn mul(self, scalar: T) -> Self {
        Self {
            vec: self.vec.iter().map(|x| *x * scalar).collect(),
        }
    }
}

impl<T: Float> Neg for Vector<T> {
    type Output = Self;

    /// Returns the additive inverse of the vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Vector;
    ///
    /// let v = Vector::new(vec![1.0, -2.0]);
    ///
    /// assert_eq!(-v, Vector::new(vec![-1.0, 2.0]));
    /// ```
    fn neg(self) -> Self {
        Self {
            vec: self.vec.iter().map(|x| -*x).collect(),
        }
    }
}

impl<T: Float> Index<usize> for Vector<T> {
    type Output = T;

    /// Returns an immutable reference to a vector component.
    fn index(&self, index: usize) -> &T {
        &self.vec[index]
    }
}

impl<T: Float> IndexMut<usize> for Vector<T> {
    /// Returns a mutable reference to a vector component.
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.vec[index]
    }
}

impl<T: Float + fmt::Display> fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self
            .vec
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "[{}]", inner)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let a = Vector::new(vec![1.0, 2.0, 3.0]);
        let b = Vector::new(vec![4.0, 5.0, 6.0]);
        assert_eq!((a + b).unwrap(), Vector::new(vec![5.0, 7.0, 9.0]));
    }

    #[test]
    fn test_dot() {
        let a = Vector::new(vec![1.0, 2.0, 3.0]);
        let b = Vector::new(vec![4.0, 5.0, 6.0]);
        assert_eq!(a.dot(&b).unwrap(), 32.0);
    }

    #[test]
    fn test_cross() {
        let a = Vector::new(vec![1.0, 0.0, 0.0]);
        let b = Vector::new(vec![0.0, 1.0, 0.0]);
        assert_eq!(a.cross(&b).unwrap(), Vector::new(vec![0.0, 0.0, 1.0]));
    }

    #[test]
    fn test_normalize() {
        let a = Vector::new(vec![3.0, 4.0]);
        let n = a.normalize().unwrap();
        assert!((n.len() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_dimension_mismatch() {
        let a = Vector::new(vec![1.0, 2.0]);
        let b = Vector::new(vec![1.0, 2.0, 3.0]);
        assert_eq!((a + b).unwrap_err(), VectorErr::DimensionMismatch);
    }
}
