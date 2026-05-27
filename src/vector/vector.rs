use crate::err::vector_err::VectorErr;
use std::fmt;
use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

pub trait VectorOps {
    fn dot(&self, other: &Self) -> Result<f64, VectorErr>
    where
        Self: Sized;
    fn len(&self) -> f64;
    fn normalize(&self) -> Result<Self, VectorErr>
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    vec: Vec<f64>,
}

impl Vector {
    pub fn new(vec: Vec<f64>) -> Self {
        Self { vec }
    }

    pub fn len(&self) -> f64 {
        self.vec.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

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

impl VectorOps for Vector {
    fn dot(&self, other: &Self) -> Result<f64, VectorErr> {
        if self.vec.len() != other.vec.len() {
            return Err(VectorErr::DimensionMismatch);
        }
        Ok(self.vec.iter().zip(&other.vec).map(|(a, b)| a * b).sum())
    }

    fn len(&self) -> f64 {
        self.vec.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    fn normalize(&self) -> Result<Self, VectorErr> {
        let len = self.len();
        if len == 0.0 {
            return Err(VectorErr::ZeroVector);
        }
        Ok(Self {
            vec: self.vec.iter().map(|x| x / len).collect(),
        })
    }
}

impl Add for Vector {
    type Output = Result<Self, VectorErr>;
    fn add(self, other: Self) -> Result<Self, VectorErr> {
        if self.vec.len() != other.vec.len() {
            return Err(VectorErr::DimensionMismatch);
        }
        Ok(Self {
            vec: self
                .vec
                .iter()
                .zip(&other.vec)
                .map(|(a, b)| a + b)
                .collect(),
        })
    }
}

impl Sub for Vector {
    type Output = Result<Self, VectorErr>;
    fn sub(self, other: Self) -> Result<Self, VectorErr> {
        if self.vec.len() != other.vec.len() {
            return Err(VectorErr::DimensionMismatch);
        }
        Ok(Self {
            vec: self
                .vec
                .iter()
                .zip(&other.vec)
                .map(|(a, b)| a - b)
                .collect(),
        })
    }
}

impl Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self {
            vec: self.vec.iter().map(|x| x * scalar).collect(),
        }
    }
}

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            vec: self.vec.iter().map(|x| -x).collect(),
        }
    }
}

impl Index<usize> for Vector {
    type Output = f64;
    fn index(&self, index: usize) -> &f64 {
        &self.vec[index]
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.vec[index]
    }
}

impl fmt::Display for Vector {
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
