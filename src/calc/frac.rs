use crate::math::Math;

pub struct Frac {
    pub num: i64,
    pub den: i64,
}

impl Frac {
    pub fn new(num: i64, den: i64) -> Self {
        Self { num, den }
    }

    pub fn to_dec(self) -> f64 {
        (self.num / self.den) as f64
    }

    pub fn normalize(mut self) -> Self {
        if self.num==0 {
            return Self { num:0, den: 1 };
        }

        if self.den < 0 {
            self.num = -self.num;
            self.den = -self.den;
        }

        let gcd = Math::gcd(self.den, self.num);

        Self {
            num: self.num / gcd,
            den: self.den / gcd
        }
    }

    pub fn reverse(self) -> Self {
        if self.num==0 {
            panic!("division by zero (reciprocal of 0)")
        }

        Self::new(self.den, self.num)
    }

    pub fn add(self, other: Self) -> Self {
        let num = self.num * other.den + self.den * other.num;
        let den = self.den * other.den;

        Self { num, den }.normalize()
    }

    pub fn min(self, other: Self) -> Self {
        let num = self.num * other.den - self.den * other.num;
        let den = self.den * other.den;

        Self { num, den }.normalize()
    }

    pub fn mul(self, other: Self) -> Self {
        let num = self.num * other.num;
        let den = self.den * other.den;

        Self { num, den }.normalize()
    }

    pub fn div(self, other: Self) -> Self {
        let num = self.num * other.den;
        let den = self.den * other.num;

        Self { num, den }.normalize()
    }
}