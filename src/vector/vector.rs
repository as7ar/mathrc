pub trait VectorOps {
    fn add(&self, other: &Self) -> Result<Self, String>
    where
        Self: Sized;
    fn sub(&self, other: &Self) -> Result<Self, String>
    where
        Self: Sized;
    fn dot(&self, other: &Self) -> Result<f64, String>
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
}

impl VectorOps for Vector {
    fn add(&self, other: &Self) -> Result<Self, String> {
        if self.vec.len() != other.vec.len() {
            return Err("Dimension Mismatched".into());
        }

        Ok(Self {
            vec: self
                .vec
                .iter()
                .zip(other.vec.iter())
                .map(|(a, b)| a + b)
                .collect(),
        })
    }

    fn sub(&self, other: &Self) -> Result<Self, String> {
        if self.vec.len() != other.vec.len() {
            return Err("Dimension Mismatched".into());
        }

        Ok(Self {
            vec: self
                .vec
                .iter()
                .zip(other.vec.iter())
                .map(|(a, b)| a - b)
                .collect(),
        })
    }

    fn dot(&self, other: &Self) -> Result<f64, String> {
        if self.vec.len() != other.vec.len() {
            return Err("Dimension Mismatched".into());
        }

        let dot = self
            .vec
            .iter()
            .zip(other.vec.iter())
            .map(|(a, b)| a * b)
            .sum();

        Ok(dot)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {}
}
