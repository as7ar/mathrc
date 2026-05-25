use std::sync::Arc;

#[derive(Clone)]
pub struct Func {
    func: Arc<dyn Fn(f64) -> f64>,
}

impl Func {
    pub fn new<F: Fn(f64) -> f64 + 'static>(func: F) -> Self {
        Self { func: Arc::new(func) }
    }

    pub fn call(&self, x: f64) -> f64 {
        (self.func)(x)
    }

    pub fn derivative(&self) -> Self {
        let h = 1e-10;
        let cloned = self.clone();
        Func::new(move |x| (cloned.call(x + h) - cloned.call(x)) / h)
    }

    pub fn integral(&self) -> Self {
        let cloned = self.clone();
        Func::new(move |x| {
            let n = 100000;
            let dx = x / n as f64;
            let mut sum = 0.0;
            for i in 0..=n {
                sum += cloned.call(i as f64 * dx) * dx;
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
        let f = Func::new(|x| x.powi(2));

        println!("{}", f.call(10.0));

        println!("{}", f.clone().derivative().call(100.0));

        let i = f.clone().integral();

        println!("{}", i.call(10.0));
    }
}
