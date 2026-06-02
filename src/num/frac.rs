use std::fmt;
use crate::math::Math;

#[derive(Clone, PartialEq)]
pub struct Frac {
    pub num: i64,
    pub den: i64,
}

impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.den==1 { return write!(f, "{}", self.num) }
        if self.num==0 { return write!(f, "0") }
        write!(f, "{}/{}", self.num, self.den)
    }
}

impl Frac {
    pub fn new(num: i64, den: i64) -> Result<Self, String> {
        if den==0 {
            return Err("\"den\" must not be 0".into())
        }
        Ok(Self { num, den })
    }

    pub fn to_dec(&self) -> f64 {
        (self.num / self.den) as f64
    }

    pub fn normalize(&self) -> Self {
        if self.num==0 { return Self { num:0, den: 1 }; }

        let (num, den) = if self.den < 0 { (-self.num, -self.den)
        } else { (self.num, self.den) };
        let gcd = Math::gcd(den, num);

        Self { num: num / gcd, den: den / gcd }
    }

    pub fn reverse(&self) -> Result<Self, String> {
        if self.num==0 {
            return Err("division by zero (reciprocal of 0)".into())
        }

        Ok(Self { num: self.den, den: self.num })
    }

    pub fn add(&self, other: &Self) -> Self {
        let num = self.num * other.den + self.den * other.num;
        let den = self.den * other.den;

        Self { num, den }.normalize()
    }

    pub fn min(&self, other: &Self) -> Self {
        let num = self.num * other.den - self.den * other.num;
        let den = self.den * other.den;

        Self { num, den }.normalize()
    }

    pub fn mul(&self, other: &Self) -> Self {
        let num = self.num * other.num;
        let den = self.den * other.den;

        Self { num, den }.normalize()
    }

    pub fn div(&self, other: &Self) -> Self {
        let num = self.num * other.den;
        let den = self.den * other.num;

        Self { num, den }.normalize()
    }
}

#[cfg(test)]
mod test {
    use crate::Frac;

    #[test]
    fn main() {
        let frac = Frac::new(2, 4).unwrap();
        println!("1/2={}", frac.normalize()); // 1/2

        let rev_frac = frac.reverse().unwrap(); // Use of moved value: `frac`
        println!("2={}", rev_frac)
    }
}