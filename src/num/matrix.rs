use std::ops::{Index, IndexMut};

use num_traits::Float;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T: Float> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Float + 'static> Matrix<T> {
    pub fn new(data: Vec<T>, rows: usize, cols: usize) -> Result<Self, String> {
        if data.len() != (rows * cols) {
            return Err("invalid matrix size".into());
        }
        Ok(Self { data, rows, cols })
    }

    pub fn get(&self, r: usize, c: usize) -> Option<T> {
        if r >= self.rows || c >= self.cols {
            return None;
        }
        Some(self.data[r * self.cols + c])
    }

    pub fn set(&mut self, r: usize, c: usize, value: T) {
        self.data[r * self.cols + c] = value;
    }

    pub fn add(&self, other: &Self) -> Result<Self, String> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err("dimension mismatch".into());
        }

        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();

        Ok(Self {
            data,
            rows: self.rows,
            cols: self.cols,
        })
    }

    pub fn mul(&self, other: &Self) -> Result<Self, String> {
        if self.cols != other.rows {
            return Err("invalid dimensions".into());
        }

        let mut data = vec![T::zero(); self.rows * other.cols];

        for r in 0..self.rows {
            for c in 0..other.cols {
                let mut sum = T::zero();
                for k in 0..self.cols {
                    sum += self[(r, k)] * other[(k, c)];
                }

                data[r * other.cols + c] = sum;
            }
        }

        Ok(Self {
            data,
            rows: self.rows,
            cols: other.cols,
        })
    }

    pub fn transpose(&self) -> Self {
        let mut data = vec![T::zero(); self.rows * self.cols];

        for r in 0..self.rows {
            for c in 0..self.cols {
                data[c * self.rows + r] = self[(r, c)];
            }
        }

        Self {
            data,
            rows: self.cols,
            cols: self.rows,
        }
    }

    pub fn identity(n: usize) -> Self {
        let mut data = vec![T::zero(); n * n];

        for i in 0..n {
            data[i * n + i] = T::one();
        }

        Self {
            data,
            rows: n,
            cols: n,
        }
    }
}

impl<T: Float> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (r, c) = index;

        &self.data[r * self.cols + c]
    }
}

impl<T: Float> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (r, c) = index;
        &mut self.data[r * self.cols + c]
    }
}

#[cfg(test)]
mod test {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn matrix() {
        let m = Matrix::new(
            vec![
                0f64, 1f64, 0f64, 1f64, 0f64, 1f64, 1f64, 0f64, 1f64, 0f64, 1f64, 0f64,
            ],
            3,
            4,
        );

        println!("{}", m.unwrap().get(1, 2).unwrap())
    }
}
