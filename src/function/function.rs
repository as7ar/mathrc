pub struct Func {
    func: Box<dyn Fn(f64) -> f64>,
}

impl Func {
    pub fn new<F>(func: F) -> Self
    where
        F: Fn(f64) -> f64 + 'static,
    {
        Self {
            func: Box::new(func),
        }
    }

    pub fn call(&self, x: f64) -> f64 {
        (self.func)(x)
    }

    pub fn derivative(self) -> Self {
        let h = 1e-10;

        Func::new(move |x| {
            (self.call(x + h) - self.call(x)) / h
        })
    }

    pub fn integral(self) -> Self {
        Func::new(move |x| {
            let n = 100000;
            let dx = x / n as f64;

            let mut sum = 0.0;

            for i in 0..=n {
                sum += self.call(i as f64 * dx) * dx;
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
        let f = Func::new(|x| x.log(10.0));

        println!("{}", f.call(100.0));

        let d = f.derivative();

        println!("{}", d.call(100.0));

        let i = Func::new(|x| x.log(10.0)).integral();

        println!("{}", i.call(100.0));
    }
}
