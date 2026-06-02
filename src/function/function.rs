use std::sync::Arc;

use num_traits::Float;

#[derive(Clone)]
pub struct Func<T: Float> {
    func: Arc<dyn Fn(T) -> T>,
}

impl<T: Float + 'static> Func<T> {
    pub fn new<F: Fn(T) -> T + 'static>(func: F) -> Self {
        Self {
            func: Arc::new(func),
        }
    }

    pub fn call(&self, x: T) -> T {
        (self.func)(x)
    }

    pub fn derivative(&self) -> Self {
        let h = T::epsilon().sqrt();
        let cloned = self.clone();

        Func::new(move |x| (cloned.call(x + h) - cloned.call(x - h)) / (h + h))
    }

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
        let f: Func<f64> = Func::new(|x| x.powi(2));

        println!("{}", f.call(10.0));

        println!("{}", f.clone().derivative().call(100.0));

        let i = f.clone().integral();

        println!("{}", i.call(10.0));
    }
}
