pub struct Math;

impl Math {
    pub const PI: f64 = std::f64::consts::PI;
    pub const E: f64 = std::f64::consts::E;
    pub const INF: f64 = f64::INFINITY;

    pub fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            (a, b) = (b, a % b);
        }

        a.abs()
    }

    pub fn lcm(a: i64, b: i64) -> i64 {
        (a / Self::gcd(a, b)) * b
    }

    pub fn factorial(n: u64) -> u64 {
        (1..=n).product()
    }
}
