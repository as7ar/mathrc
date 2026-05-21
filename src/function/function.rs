pub struct Func<F> where F: Fn(f64) -> f64,{
    func: F,
}

impl<F> Func<F> where F: Fn(f64) -> f64, {
    pub fn new(func: F) -> Self {
        Self { func }
    }

    pub fn call(&self, x: f64) -> f64 {
        (self.func)(x)
    }

    pub fn derivative(&self) -> Self {
        let h = 1e-10;
        Func::new(|x| {
            (self.call(x+h)-self.call(x))/h
        })
    }

    pub fn integral(&self) -> Self {
        let f = self.func.clone();

        Func::new(move |x| {
            let d = 1e10;
            let dx = x / d;

            let mut sum = 0.0;

            for i in 0.0..=d {
                sum += f(i * dx) * dx;
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

        println!("{}", f.call(1.0))
    }
}