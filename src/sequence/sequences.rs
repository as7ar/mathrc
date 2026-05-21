pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn lucas(n: u64) -> u64 {
    match n {
        0 => 2,
        1 => 1,
        _ => lucas(n - 1) + lucas(n - 2),
    }
}

pub fn triangular(n: u64) -> u64 {
    n * (n + 1) / 2
}

pub fn square(n: u64) -> u64 {
    n * n
}

pub fn pentagonal(n: u64) -> u64 {
    n * (3 * n - 1) / 2
}

pub fn hexagonal(n: u64) -> u64 {
    n * (2 * n - 1)
}

pub fn arithmetic(n: u64, a1: i64, d: i64) -> i64 {
    a1 + (n as i64 - 1) * d
}

pub fn geometric(n: u64, a1: f64, r: f64) -> f64 {
    a1 * r.powi((n - 1) as i32)
}

pub fn harmonic(n: u64) -> f64 {
    1.0 / n as f64
}

pub fn catalan(n: u64) -> u64 {
    factorial(2 * n) / (factorial(n + 1) * factorial(n))
}

pub fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

pub fn collatz(n: u64) -> u64 {
    match n % 2 {
        0 => n / 2,
        _ => 3 * n + 1,
    }
}

pub fn mersenne(n: u32) -> u64 {
    2u64.pow(n) - 1
}