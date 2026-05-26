#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    vec: Vec<f64>,
}

impl Vector {
    pub fn new(vec: Vec<f64>) -> Self {
        Self { vec }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {}
}
