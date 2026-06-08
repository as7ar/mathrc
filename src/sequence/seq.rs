/// A mathematical sequence.
///
/// A sequence is defined by a function that maps a natural number
/// index to a floating-point value.
///
/// # Examples
///
/// ```rust
/// use mathrc::Seq;
///
/// let seq = Seq::new(|n| n as f64);
///
/// assert_eq!(seq.nth(5), 5.0);
/// ```
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
    /// Creates a new sequence.
    ///
    /// # Arguments
    ///
    /// * `seq` - Function defining the sequence.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Seq;
    ///
    /// let seq = Seq::new(|n| (n * n) as f64);
    /// ```
    pub fn new(seq: S) -> Self {
        Self { seq }
    }

    /// Returns the value of the sequence at index `n`.
    ///
    /// # Arguments
    ///
    /// * `n` - Sequence index.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Seq;
    ///
    /// let seq = Seq::new(|n| (n * n) as f64);
    ///
    /// assert_eq!(seq.nth(3), 9.0);
    /// ```
    pub fn nth(&self, n: u64) -> f64 {
        (self.seq)(n)
    }

    /// Computes the sum of sequence terms from `k` to `n` (inclusive).
    ///
    /// This evaluates:
    ///
    /// ```text
    /// a(k) + a(k+1) + ... + a(n)
    /// ```
    ///
    /// # Arguments
    ///
    /// * `k` - Starting index.
    /// * `n` - Ending index.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Seq;
    ///
    /// let seq = Seq::new(|n| n as f64);
    ///
    /// assert_eq!(seq.sum(1, 5), 15.0);
    /// ```
    pub fn sum(&self, k: u64, n: u64) -> f64 {
        (k..=n).map(|i| (self.seq)(i)).sum()
    }
}
