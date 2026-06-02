pub struct Seq<S>
where
    S: Fn(u64) -> f64,
{
    seq: S,
}

impl<S> Seq<S>
where
    S: Fn(u64) -> f64,
{
    pub fn new(seq: S) -> Self {
        Self { seq }
    }

    pub fn nth(&self, n: u64) -> f64 {
        (self.seq)(n)
    }

    pub fn sum(&self, k: u64, n: u64) -> f64 {
        (k..=n).map(|i| (self.seq)(i)).sum()
    }
}
