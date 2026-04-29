use std::string::ToString;
use crate::Math;

impl Math {
    pub const PI: f64 = std::f64::consts::PI;
    pub const E: f64 = std::f64::consts::E;
    pub const INF: &str = "∞";

    pub fn add<T: Into<f64>, U: Into<f64>>(
        a: T, b: U
    ) -> f64 {
        a.into() + b.into()
    }

    pub fn sub<T: Into<f64>, U: Into<f64>>(
        a: T, b: U
    ) -> f64 {
        a.into() - b.into()
    }

    pub fn mul<T: Into<f64>, U: Into<f64>>(
        a: T, b: U
    ) -> f64 {
        a.into() * b.into()
    }

    pub fn div<T: Into<f64>, U: Into<f64>>(
        a: T, b: U
    ) -> f64 {
        a.into() / b.into()
    }

    pub fn remain<T: Into<f64>, U: Into<f64>>(
        a: T, b: U
    ) -> f64 {
        a.into() % b.into()
    }

    pub fn sqrt<T: Into<f64>>(a: T) -> Option<f64> {
        let a = a.into();
        if a < 0.0 { return None; }
        Some(a.sqrt())
    }

    pub fn abs<T: Into<f64>>(a: T) -> f64 {
        a.into().abs()
    }

    pub fn pow<T: Into<f64>, U: Into<f64>>(base: T, exp: U) -> f64 {
        base.into().powf(exp.into())
    }

    pub fn log<T: Into<f64>, U: Into<f64>>(x: T, base: U) -> Option<f64> {
        let x = x.into();
        let base = base.into();
        if x <= 0.0 || base <= 0.0 || base == 1.0 { return None; }
        Some(x.log(base))
    }

    fn gcd<T: Into<i64>, U: Into<i64>>(a: T, b: U) -> i64 {
        let mut a = a.into();
        let mut b = b.into();

        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a.abs()
    }

    pub fn lcm<T: Into<i64>, U: Into<i64>>(a: T, b: U) -> i64 {
        let a = a.into();
        let b = b.into();

        (a / Self::gcd(a, b)) * b
    }

    pub fn dec_to_frac<T: Into<f64>>(dec: T) -> String {

        let x = dec.into();

        if x.is_nan() { return "NaN".to_string(); }
        if x.is_infinite() { return if x > 0.0 { "∞".to_string() } else { "-∞".to_string() }; }

        let sign = if x < 0.0 { "-" } else { "" };
        let x = x.abs();
        let decimal = x.fract();

        if decimal == 0.0 { return format!("{}{}", sign, x.trunc() as i64); }

        let precision = 1_000_000_000_000_000_000_i64;
        let numerator = (x * precision as f64).round() as i64;
        let denominator = precision;
        let gcd =  Self::gcd(numerator.abs(), denominator);

        let n = numerator / gcd;
        let d = denominator / gcd;
        if d == 1 { return format!("{}{}", sign, n); }
        format!("{}{}/{}", sign, n, d)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {

        println!("{}", Math::add(12, 21).to_string()); // 33
    }
}