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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3d<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vector3d<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn to_vec(&self) -> Vector<T> {
        Vector::new(vec![self.x, self.y, self.z])
    }
}

impl<T> VectorOps<T> for Vector3d<T>
where
    T: Float + Sum<T>,
{
    fn dot(&self, other: &Self) -> Result<T, VectorErr> {
        Ok(self.x * other.x + self.y + other.y + self.z + other.z)
    }

    fn magnitude(&self) -> T {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn normalize(&self) -> Result<Self, VectorErr> {
        let mag = self.len();
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
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl<T: Float + fmt::Display> fmt::Display for Vector3d<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}
