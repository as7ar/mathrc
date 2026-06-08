//! A generic 2-dimensional vector implementation.
//!
//! This module provides the [`Vector2d`] type along with common vector
//! operations such as addition, subtraction, scalar multiplication,
//! dot products, normalization, and conversion to a generic [`Vector`].
//!
//! # Examples
//!
//! ```rust
//! use mathrc::Vector2d;
//!
//! let a = Vector2d::new(1.0, 2.0);
//! let b = Vector2d::new(3.0, 4.0);
//!
//! let c = a + b;
//! assert_eq!(c, Vector2d::new(4.0, 6.0));
//! ```
//!
//! ```rust
//! use mathrc::{Vector2d, VectorOps};
//!
//! let v = Vector2d::new(3.0, 4.0);
//! let normalized = v.normalize().unwrap();
//!
//! assert!((normalized.magnitude() - 1.0).abs() < 1e-10);
//! ```

use num_traits::Float;

use crate::err::VectorErr;
use crate::vector::{Vector, VectorOps};
use std::fmt;
use std::iter::Sum;
use std::ops::{Add, Mul, Neg, Sub};

/// A generic 2-dimensional vector.
///
/// # Type Parameters
///
/// * `T` - Floating-point numeric type.
///
/// # Examples
///
/// ```rust
/// use mathrc::Vector2d;
///
/// let v = Vector2d::new(1.0, 2.0);
///
/// assert_eq!(v.x, 1.0);
/// assert_eq!(v.y, 2.0);
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2d<T: Float> {
    /// X component.
    pub x: T,

    /// Y component.
    pub y: T,
}

impl<T: Float> Vector2d<T> {
    /// Creates a new 2D vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Vector2d;
    ///
    /// let v = Vector2d::new(3.0, 4.0);
    ///
    /// assert_eq!(v.x, 3.0);
    /// assert_eq!(v.y, 4.0);
    /// ```
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Converts this vector into a generic [`Vector`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Vector2d;
    ///
    /// let v = Vector2d::new(1.0, 2.0);
    /// let vec = v.to_vec();
    ///
    /// assert_eq!(vec[0], 1.0);
    /// assert_eq!(vec[1], 2.0);
    /// ```
    pub fn to_vec(&self) -> Vector<T> {
        Vector::new(vec![self.x, self.y])
    }
}

impl<T> VectorOps<T> for Vector2d<T>
where
    T: Float + Sum<T>,
{
    /// Computes the dot product of two vectors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::{Vector2d, VectorOps};
    ///
    /// let a = Vector2d::new(1.0, 2.0);
    /// let b = Vector2d::new(3.0, 4.0);
    ///
    /// assert_eq!(a.dot(&b).unwrap(), 11.0);
    /// ```
    fn dot(&self, other: &Self) -> Result<T, VectorErr> {
        Ok(self.x * other.x + self.y * other.y)
    }

    /// Returns the Euclidean length of the vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::{Vector2d, VectorOps};
    ///
    /// let v = Vector2d::new(3.0, 4.0);
    ///
    /// assert_eq!(v.magnitude(), 5.0);
    /// ```
    fn magnitude(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns a normalized version of the vector.
    ///
    /// # Errors
    ///
    /// Returns [`VectorErr::ZeroVector`] if the vector length is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::{Vector2d, VectorOps};
    ///
    /// let v: Vector2d<f64> = Vector2d::new(3.0, 4.0);
    /// let n: Vector2d<f64> = v.normalize().unwrap();
    ///
    /// assert!((n.magnitude() - 1.0).abs() < 1e-10);
    /// ```
    fn normalize(&self) -> Result<Self, VectorErr> {
        let mag = self.magnitude();

        if mag == T::zero() {
            return Err(VectorErr::ZeroVector);
        }

        Ok(Self {
            x: self.x / mag,
            y: self.y / mag,
        })
    }
}

impl<T: Float> Add for Vector2d<T> {
    type Output = Self;

    /// Adds two vectors component-wise.
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Float> Sub for Vector2d<T> {
    type Output = Self;

    /// Subtracts two vectors component-wise.
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Float> Mul<T> for Vector2d<T> {
    type Output = Self;

    /// Multiplies a vector by a scalar.
    fn mul(self, scalar: T) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T: Float> Neg for Vector2d<T> {
    type Output = Self;

    /// Negates both vector components.
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Float + fmt::Display> fmt::Display for Vector2d<T> {
    /// Formats the vector as `[x, y]`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let a = Vector2d::new(1.0, 2.0);
        let b = Vector2d::new(3.0, 4.0);
        assert_eq!(a + b, Vector2d::new(4.0, 6.0));
    }

    #[test]
    fn test_dot() {
        let a = Vector2d::new(1.0, 2.0);
        let b = Vector2d::new(3.0, 4.0);
        assert_eq!(a.dot(&b).unwrap(), 11.0);
    }

    #[test]
    fn test_normalize() {
        let a = Vector2d::new(3.0, 4.0);
        let n = a.normalize().unwrap();
        assert!((n.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_zero_normalize() {
        let a = Vector2d::new(0.0, 0.0);
        assert_eq!(a.normalize().unwrap_err(), VectorErr::ZeroVector);
    }
}
