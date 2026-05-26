use crate::vector::vector::Vector;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2d {
    x: f64,
    y: f64,
}

impl Vector2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn to_vec(&self) -> Vector {
        Vector::new(vec![self.x, self.y])
    }
}
