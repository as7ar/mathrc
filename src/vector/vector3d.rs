//! A generic 3-dimensional vector implementation.
//!
//! This module provides the [`Vector3d`] type and common vector operations,
//! including vector arithmetic, dot products, cross products,
//! normalization, and conversion to a generic [`Vector`].
//!
//! # Examples
//!
//! ```rust
//! use mathrc::vector::Vector3d;
//!
//! let a = Vector3d::new(1.0, 2.0, 3.0);
//! let b = Vector3d::new(4.0, 5.0, 6.0);
//!
//! let c = a + b;
//!
//! assert_eq!(c, Vector3d::new(5.0, 7.0, 9.0));
//! ```
//!
//! ```rust
//! use mathrc::vector::{Vector3d, VectorOps};
//!
//! let v = Vector3d::new(3.0, 4.0, 0.0);
//!
//! assert_eq!(v.magnitude(), 5.0);
//! ```

use num_traits::Float;
use std::iter::Sum;
use std::{
    fmt,
    ops::{Add, Mul, Neg, Sub},
};

use crate::{
    err::VectorErr,
    vector::{Vector, VectorOps},
};

/// A generic 3-dimensional vector.
///
/// # Type Parameters
///
/// * `T` - Floating-point numeric type.
///
/// # Examples
///
/// ```rust
/// use mathrc::vector::Vector3d;
///
/// let v = Vector3d::new(1.0, 2.0, 3.0);
///
/// assert_eq!(v.x, 1.0);
/// assert_eq!(v.y, 2.0);
/// assert_eq!(v.z, 3.0);
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3d<T: Float> {
    /// X component.
    pub x: T,

    /// Y component.
    pub y: T,

    /// Z component.
    pub z: T,
}

impl<T: Float> Vector3d<T> {
    /// Creates a new 3D vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::vector::Vector3d;
    ///
    /// let v = Vector3d::new(1.0, 2.0, 3.0);
    ///
    /// assert_eq!(v.x, 1.0);
    /// ```
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Converts this vector into a generic [`Vector`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::vector::Vector3d;
    ///
    /// let v = Vector3d::new(1.0, 2.0, 3.0);
    /// let vec = v.to_vec();
    ///
    /// assert_eq!(vec[0], 1.0);
    /// assert_eq!(vec[1], 2.0);
    /// assert_eq!(vec[2], 3.0);
    /// ```
    pub fn to_vec(&self) -> Vector<T> {
        Vector::new(vec![self.x, self.y, self.z])
    }

    /// Computes the cross product of two 3D vectors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::vector::Vector3d;
    ///
    /// let a = Vector3d::new(1.0, 0.0, 0.0);
    /// let b = Vector3d::new(0.0, 1.0, 0.0);
    ///
    /// assert_eq!(
    ///     a.cross(&b),
    ///     Vector3d::new(0.0, 0.0, 1.0)
    /// );
    /// ```
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl<T> VectorOps<T> for Vector3d<T>
where
    T: Float + Sum<T>,
{
    /// Computes the dot product of two vectors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::vector::{Vector3d, VectorOps};
    ///
    /// let a = Vector3d::new(1.0, 2.0, 3.0);
    /// let b = Vector3d::new(4.0, 5.0, 6.0);
    ///
    /// assert_eq!(a.dot(&b).unwrap(), 32.0);
    /// ```
    fn dot(&self, other: &Self) -> Result<T, VectorErr> {
        Ok(self.x * other.x + self.y * other.y + self.z * other.z)
    }

    /// Returns the Euclidean magnitude of the vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::vector::{Vector3d, VectorOps};
    ///
    /// let v = Vector3d::new(1.0, 2.0, 2.0);
    ///
    /// assert_eq!(v.magnitude(), 3.0);
    /// ```
    fn magnitude(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Returns a normalized vector.
    ///
    /// # Errors
    ///
    /// Returns [`VectorErr::ZeroVector`] if the vector magnitude is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::vector::{Vector3d, VectorOps};
    ///
    /// let v = Vector3d::new(3.0, 4.0, 0.0);
    /// let n = v.normalize().unwrap();
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
            z: self.z / mag,
        })
    }
}

impl<T: Float> Add for Vector3d<T> {
    type Output = Self;

    /// Adds two vectors component-wise.
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Float> Sub for Vector3d<T> {
    type Output = Self;

    /// Subtracts two vectors component-wise.
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Float> Mul<T> for Vector3d<T> {
    type Output = Self;

    /// Multiplies a vector by a scalar.
    fn mul(self, scalar: T) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl<T: Float> Neg for Vector3d<T> {
    type Output = Self;

    /// Negates all vector components.
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Float + fmt::Display> fmt::Display for Vector3d<T> {
    /// Formats the vector as `[x, y, z]`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let a = Vector3d::new(1.0, 2.0, 3.0);
        let b = Vector3d::new(3.0, 4.0, 5.0);

        assert_eq!(a + b, Vector3d::new(4.0, 6.0, 8.0));
    }

    #[test]
    fn test_dot() {
        let a = Vector3d::new(1.0, 2.0, 3.0);
        let b = Vector3d::new(4.0, 5.0, 6.0);

        assert_eq!(a.dot(&b).unwrap(), 32.0);
    }

    #[test]
    fn test_cross() {
        let a = Vector3d::new(1.0, 0.0, 0.0);
        let b = Vector3d::new(0.0, 1.0, 0.0);

        assert_eq!(a.cross(&b), Vector3d::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_normalize() {
        let a = Vector3d::new(3.0, 4.0, 0.0);
        let n = a.normalize().unwrap();

        assert!((n.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_zero_normalize() {
        let a = Vector3d::new(0.0, 0.0, 0.0);

        assert_eq!(a.normalize().unwrap_err(), VectorErr::ZeroVector);
    }
}
