//! Provides a rational number type and basic arithmetic operations.

use crate::math::Math;
use std::fmt;

/// Represents a rational number.
///
/// A fraction consists of a numerator and a denominator.
///
/// # Examples
///
/// ```
/// use mathrc::Frac;
///
/// let frac = Frac::new(1, 2).unwrap();
///
/// assert_eq!(frac.num, 1);
/// assert_eq!(frac.den, 2);
/// ```
#[derive(Clone, PartialEq)]
pub struct Frac {
    /// The numerator of the fraction.
    pub num: i64,
    /// The denominator of the fraction.
    pub den: i64,
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

impl Frac {
    /// Creates a new fraction.
    ///
    /// # Errors
    ///
    /// Returns an error if `den` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathrc::Frac;
    ///
    /// let frac = Frac::new(2, 4).unwrap();
    /// ```
    pub fn new(num: i64, den: i64) -> Result<Self, String> {
        if den == 0 {
            return Err("\"den\" must not be 0".into());
        }
        Ok(Self { num, den })
    }

    /// Converts the fraction into a decimal value.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathrc::Frac;
    ///
    /// let frac = Frac::new(1, 2).unwrap();
    ///
    /// assert_eq!(frac.to_dec(), 0.5);
    /// ```
    pub fn to_dec(&self) -> f64 {
        self.num as f64 / self.den as f64
    }

    /// Returns the fraction in its reduced form.
    ///
    /// The sign is normalized so that the denominator is always positive.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathrc::Frac;
    ///
    /// let frac = Frac::new(2, 4).unwrap();
    ///
    /// assert_eq!(frac.normalize().to_string(), "1/2");
    /// ```
    pub fn normalize(&self) -> Self {
        if self.num == 0 {
            return Self { num: 0, den: 1 };
        }

        let (num, den) = if self.den < 0 {
            (-self.num, -self.den)
        } else {
            (self.num, self.den)
        };
        let gcd = Math::gcd(den, num);

        Self {
            num: num / gcd,
            den: den / gcd,
        }
    }

    /// Returns the reciprocal of the fraction.
    ///
    /// # Errors
    ///
    /// Returns an error if the numerator is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathrc::Frac;
    ///
    /// let frac = Frac::new(2, 3).unwrap();
    ///
    /// assert_eq!(frac.reverse().unwrap().to_string(), "3/2");
    /// ```
    pub fn reverse(&self) -> Result<Self, String> {
        if self.num == 0 {
            return Err("division by zero (reciprocal of 0)".into());
        }

        Ok(Self {
            num: self.den,
            den: self.num,
        })
    }

    /// Adds two fractions.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathrc::Frac;
    ///
    /// let a = Frac::new(1, 2).unwrap();
    /// let b = Frac::new(1, 3).unwrap();
    ///
    /// assert_eq!(a.add(&b).to_string(), "5/6");
    /// ```
    pub fn add(&self, other: &Self) -> Self {
        let num = self.num * other.den + self.den * other.num;
        let den = self.den * other.den;

        Self { num, den }.normalize()
    }

    /// Subtracts another fraction from this fraction.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathrc::Frac;
    ///
    /// let a = Frac::new(3, 4).unwrap();
    /// let b = Frac::new(1, 4).unwrap();
    ///
    /// assert_eq!(a.sub(&b).to_string(), "1/2");
    /// ```
    pub fn sub(&self, other: &Self) -> Self {
        let num = self.num * other.den - self.den * other.num;
        let den = self.den * other.den;

        Self { num, den }.normalize()
    }

    /// Multiplies two fractions.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathrc::Frac;
    ///
    /// let a = Frac::new(2, 3).unwrap();
    /// let b = Frac::new(3, 4).unwrap();
    ///
    /// assert_eq!(a.mul(&b).to_string(), "1/2");
    /// ```
    pub fn mul(&self, other: &Self) -> Self {
        let num = self.num * other.num;
        let den = self.den * other.den;

        Self { num, den }.normalize()
    }

    /// Divides this fraction by another fraction.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathrc::Frac;
    ///
    /// let a = Frac::new(1, 2).unwrap();
    /// let b = Frac::new(1, 4).unwrap();
    ///
    /// assert_eq!(a.div(&b).to_string(), "2");
    /// ```
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
        println!("2={}", rev_frac);

        println!("{}". )
    }
}
