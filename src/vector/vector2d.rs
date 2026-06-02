use num_traits::Float;

use crate::err::VectorErr;
use crate::vector::{Vector, VectorOps};
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2d<T: Float> {
    pub x: T,
    pub y: T,
}

impl<T: Float> Vector2d<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn to_vec(&self) -> Vector {
        Vector::new(vec![self.x, self.y])
    }
}

impl<T: Float> VectorOps<T> for Vector2d<T> {
    fn dot(&self, other: &Self) -> Result<T, VectorErr> {
        Ok(self.x * other.x + self.y * other.y)
    }

    fn len(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalize(&self) -> Result<Self, VectorErr> {
        let mag = self.len();
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
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Float> Sub for Vector2d<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Float> Mul<T> for Vector2d<T> {
    type Output = Self;
    fn mul(self, scalar: T) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T: Float> Neg for Vector2d<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Float + fmt::Display> fmt::Display for Vector2d<T> {
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
        assert!((n.len() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_zero_normalize() {
        let a = Vector2d::new(0.0, 0.0);
        assert_eq!(a.normalize().unwrap_err(), VectorErr::ZeroVector);
    }
}
