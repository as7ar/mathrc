pub struct Math;

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

    // 최대 공약수
    pub fn gcd<T: Into<i64>, U: Into<i64>>(a: T, b: U) -> i64 {
        let mut a = a.into();
        let mut b = b.into();

        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a.abs()
    }

    // 최소 공배수
    pub fn lcm<T: Into<i64>, U: Into<i64>>(a: T, b: U) -> i64 {
        let a = a.into();
        let b = b.into();

        (a / Self::gcd(a, b)) * b
    }
}