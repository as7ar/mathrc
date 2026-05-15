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