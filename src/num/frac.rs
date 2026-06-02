use num_traits::{FromPrimitive, PrimInt};

use crate::math::Math;
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Frac<T: PrimInt> {
    pub num: T,
    pub den: T,
}

impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.den == 1 {
            return write!(f, "{}", self.num);
        }
        if self.num == 0 {
            return write!(f, "0");
        }
        write!(f, "{}/{}", self.num, self.den)
    }
}

impl<T: PrimInt + FromPrimitive + 'static> Frac<T> {
    pub fn new(num: T, den: T) -> Result<Self, String> {
        if den == T::zero() {
            return Err("\"den\" must not be 0".into());
        }
        Ok(Self { num, den })
    }

    pub fn to_dec(&self) -> Option<f64> {
        Some(self.num.to_f64()? / self.den.to_f64()?)
    }

    pub fn normalize(&self) -> Option<Self> {
        if self.num == T::zero() {
            return Some(Self {
                num: T::zero(),
                den: T::one(),
            });
        }

        let (num, den) = if self.den < T::zero() {
            (-self.num, -self.den)
        } else {
            (self.num, self.den)
        };
        let gcd = Math::gcd(den.to_i64()?, den.to_i64()?);

        Some(Self {
            num: T::from_i64(num.to_i64()? / gcd)?,
            den: T::from_i64(den.to_i64()? / gcd)?,
        })
    }

    pub fn reverse(&self) -> Result<Self, String> {
        if self.num == T::zero() {
            return Err("division by zero (reciprocal of 0)".into());
        }

        Ok(Self {
            num: self.den,
            den: self.num,
        })
    }

    pub fn add(&self, other: &Self) -> Self {
        let num = self.num * other.den + self.den * other.num;
        let den = self.den * other.den;

        Self { num, den }.normalize()?
    }

    pub fn min(&self, other: &Self) -> Self {
        let num = self.num * other.den - self.den * other.num;
        let den = self.den * other.den;

        Self { num, den }.normalize()?
    }

    pub fn mul(&self, other: &Self) -> Self {
        let num = self.num * other.num;
        let den = self.den * other.den;

        Self { num, den }.normalize()?
    }

    pub fn div(&self, other: &Self) -> Self {
        let num = self.num * other.den;
        let den = self.den * other.num;

        Self { num, den }.normalize()?
    }
}

#[cfg(test)]
mod test {
    use crate::calc::frac::Frac;

    #[test]
    fn main() {
        let frac = Frac::new(2, 4).unwrap();
        println!("1/2={}", frac.normalize()); // 1/2

        let rev_frac = frac.reverse().unwrap(); // Use of moved value: `frac`
        println!("2={}", rev_frac)
    }
}
