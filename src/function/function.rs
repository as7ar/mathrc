//! Provides numerical operations for mathematical functions.
//!
//! Supported operations:
//! - Function evaluation
//! - Numerical differentiation
//! - Numerical integration

use num_traits::Float;
use std::sync::Arc;

/// Represents a mathematical function `f(x)`.
///
/// # Examples
///
/// ```
/// let f = Func::new(|x: f64| x * x);
///
/// assert_eq!(f.call(3.0), 9.0);
/// ```
#[derive(Clone)]
pub struct Func<T: Float> {
    func: Arc<dyn Fn(T) -> T>,
}

impl<T: Float + 'static> Func<T> {
    /// Creates a new function.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = Func::new(|x: f64| x + 1.0);
    ///
    /// assert_eq!(f.call(2.0), 3.0);
    /// ```
    pub fn new<F: Fn(T) -> T + 'static>(func: F) -> Self {
        Self {
            func: Arc::new(func),
        }
    }

    /// Evaluates the function at the given point.
    pub fn call(&self, x: T) -> T {
        (self.func)(x)
    }

    /// Returns the numerical derivative of the function.
    ///
    /// Uses the central difference method.
    pub fn derivative(&self) -> Self {
        let h = T::epsilon().sqrt();
        let cloned = self.clone();

        Func::new(move |x| (cloned.call(x + h) - cloned.call(x - h)) / (h + h))
    }

    /// Returns the numerical antiderivative of the function.
    ///
    /// The integral is approximated using numerical integration
    /// with a lower bound of `0`.
    pub fn integral(&self) -> Self {
        let cloned = self.clone();

        Func::new(move |x| {
            let n = 1000usize;
            let dx = x / T::from(n).unwrap();
            let mut sum = T::zero();

            for i in 0..n {
                let t = T::from(i).unwrap() * dx;
                sum = sum + cloned.call(t) * dx;
            }
            sum
        })
    }
}

#[cfg(test)]
mod test {
    use crate::function::function::Func;

    #[test]
    fn function() {
        let f: Func<f64> = Func::new(|x: f64| x.powi(2));

        println!("{}", f.call(10.0));

        println!("{}", f.clone().derivative().call(100.0));

        let i = f.clone().integral();

        println!("{}", i.call(10.0));
    }
}
