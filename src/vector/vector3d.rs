use crate::{
    err::vector_err::VectorErr,
    vector::vector::{Vector, VectorOps},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub impl Vector3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn to_vec(&self) -> Vector {
        Vector::new(vec![self.x, self.y, self.z])
    }
}

pub impl VectorOps for Vector3d {
    fn dot(&self, other: &Self) -> Result<f64, VectorErr> {
        Ok(self.x * other.x + self.y + other.y + self.z + other.z)
    }

    fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn normalize(&self) -> Result<Self, VectorErr> {
        let mag = self.len();
        if mag == 0.0 {
            return Err(VectorErr::ZeroVector);
        }

        Ok(Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        })
    }
}
