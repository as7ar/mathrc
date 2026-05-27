use crate::err::vector_err::VectorErr;
use crate::vector::vector::{Vector, VectorOps};
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2d {
    pub x: f64,
    pub y: f64,
}

impl Vector2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn to_vec(&self) -> Vector {
        Vector::new(vec![self.x, self.y])
    }
}

impl VectorOps for Vector2d {
    fn dot(&self, other: &Self) -> Result<f64, VectorErr> {
        Ok(self.x * other.x + self.y * other.y)
    }

    fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalize(&self) -> Result<Self, VectorErr> {
        let mag = self.len();
        if mag == 0.0 {
            return Err(VectorErr::ZeroVector);
        }
        Ok(Self {
            x: self.x / mag,
            y: self.y / mag,
        })
    }
}

impl Add for Vector2d {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2d {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Vector2d {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Neg for Vector2d {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl fmt::Display for Vector2d {
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
