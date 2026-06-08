//! Mathematical constants and utility functions.
//!
//! This module provides commonly used mathematical constants and
//! helper functions such as greatest common divisor (GCD),
//! least common multiple (LCM), and factorial calculations.
//!
//! # Examples
//!
//! ```rust
//! use mathrc::Math;
//!
//! assert_eq!(Math::gcd(12, 18), 6);
//! assert_eq!(Math::lcm(12, 18), 36);
//! assert_eq!(Math::factorial(5), 120);
//! ```
//!
//! ```rust
//! use mathrc::Math;
//!
//! let circumference = 2.0 * Math::PI * 5.0;
//! assert!(circumference > 31.0);
//! ```

use num_bigint::BigUint;
use num_traits::One;

/// Mathematical utility type.
///
/// This type serves as a namespace for mathematical constants
/// and helper functions.
pub struct Math;

impl Math {
    /// Archimedes' constant π.
    pub const PI: f64 = std::f64::consts::PI;

    /// Euler's number e.
    pub const E: f64 = std::f64::consts::E;

    /// Positive infinity.
    pub const INF: f64 = f64::INFINITY;

    /// Computes the greatest common divisor (GCD) of two integers.
    ///
    /// Uses the Euclidean algorithm.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Math;
    ///
    /// assert_eq!(Math::gcd(12, 18), 6);
    /// assert_eq!(Math::gcd(-12, 18), 6);
    /// ```
    pub fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            (a, b) = (b, a % b);
        }

        a.abs()
    }

    /// Computes the least common multiple (LCM) of two integers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Math;
    ///
    /// assert_eq!(Math::lcm(12, 18), 36);
    /// ```
    pub fn lcm(a: i64, b: i64) -> i64 {
        (a / Self::gcd(a, b)) * b
    }

    /// Computes the factorial of a non-negative integer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Math;
    ///
    /// assert_eq!(Math::factorial(0), 1);
    /// assert_eq!(Math::factorial(5), 120);
    /// ```
    pub fn factorial(n: u64) -> Option<u64> {
        let mut result = 1u64;

        for i in 1..=n {
            result = result.checked_mul(i)?;
        }

        Some(result)
    }
}
