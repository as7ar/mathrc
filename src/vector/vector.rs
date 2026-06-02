use num_traits::Float;

use crate::err::VectorErr;
use std::fmt;
use std::iter::Sum;
use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

pub trait VectorOps<T>
where
    T: Float + Sum,
{
    fn dot(&self, other: &Self) -> Result<T, VectorErr>
    where
        Self: Sized;
    fn len(&self) -> T;
    fn normalize(&self) -> Result<Self, VectorErr>
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T: Float> {
    vec: Vec<T>,
}

impl<T: Float> Vector<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Self { vec }
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

impl<T: Float> VectorOps<T> for Vector<T> {
    fn dot(&self, other: &Self) -> Result<T, VectorErr> {
        if self.vec.len() != other.vec.len() {
            return Err(VectorErr::DimensionMismatch);
        }
        Ok(self.vec.iter().zip(&other.vec).map(|(a, b)| *a * *b).sum())
    }

    fn len(&self) -> T {
        self.vec.iter().map(|x| *x * *x).sum::<T>().sqrt()
    }

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
    fn mul(self, scalar: T) -> Self {
        Self {
            vec: self.vec.iter().map(|x| *x * scalar).collect(),
        }
    }
}

impl<T: Float> Neg for Vector<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            vec: self.vec.iter().map(|x| -*x).collect(),
        }
    }
}

impl<T: Float> Index<usize> for Vector<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.vec[index]
    }
}

impl<T: Float> IndexMut<usize> for Vector<T> {
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
