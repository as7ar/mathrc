use crate::Math;

pub struct Trigonometric;

impl Trigonometric {
    pub fn deg_to_rad<T: Into<f64>>(a: T) -> f64 {
        Math::mul(a.into(), Math::div(Math::PI, 180))
    }

    pub fn rad_to_string<T: Into<f64>>(a: T) -> String {
        format!("{}*π", Math::dec_to_frac(Math::div(a.into(), Math::PI)))
    }
}

#[cfg(test)]
mod test {
    use crate::function::trigonometric::Trigonometric;

    #[test]
    fn deg2rad() {
        println!("{}", Trigonometric::rad_to_string(Trigonometric::deg_to_rad(60)))
    }
}