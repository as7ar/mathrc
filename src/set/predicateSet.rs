#[derive(Debug, Clone, )]
pub struct PredicateSet<T> where T: Fn(f64) -> bool,  {
    condition: T
}

impl<T> PredicateSet<T> where T: Fn(f64) -> bool {
    pub fn new(condition: T) -> Self {
        Self { condition }
    }

    pub fn contains(&self, n: f64) -> bool {
        (self.condition)(n)
    }

    pub fn is_subset(&self, other: Self) -> bool {
        // todo: Q.E.D.
        false
    }
}