/// A set defined by a predicate.
///
/// Instead of explicitly storing elements, a predicate set determines
/// membership by evaluating a condition.
///
/// This is useful for representing infinite or dynamically-defined sets,
/// such as:
///
/// - All even numbers
/// - All positive numbers
/// - All values greater than 10
///
/// # Examples
///
/// ```rust
/// use mathrc::PredicateSet;
///
/// let positive = PredicateSet::new(|x| x > 0.0);
///
/// assert!(positive.contains(5.0));
/// assert!(!positive.contains(-5.0));
/// ```
#[derive(Debug, Clone)]
pub struct PredicateSet<T>
where
    T: Fn(f64) -> bool,
{
    condition: T,
}

impl<T> PredicateSet<T>
where
    T: Fn(f64) -> bool,
{
    /// Creates a new predicate set.
    ///
    /// # Arguments
    ///
    /// * `condition` - A predicate used to determine whether a value
    ///   belongs to the set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::PredicateSet;
    ///
    /// let even = PredicateSet::new(|x| x % 2.0 == 0.0);
    ///
    /// assert!(even.contains(4.0));
    /// ```
    pub fn new(condition: T) -> Self {
        Self { condition }
    }

    /// Returns `true` if the given value belongs to the set.
    ///
    /// # Arguments
    ///
    /// * `n` - Value to test.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::PredicateSet;
    ///
    /// let positive = PredicateSet::new(|x| x > 0.0);
    ///
    /// assert!(positive.contains(10.0));
    /// assert!(!positive.contains(-10.0));
    /// ```
    pub fn contains(&self, n: f64) -> bool {
        (self.condition)(n)
    }

    /// Determines whether this set is a subset of another predicate set.
    ///
    /// # Deprecated
    ///
    /// Predicate-defined sets generally cannot be compared for subset
    /// relationships without additional mathematical constraints or
    /// a finite domain of evaluation.
    ///
    /// This method currently returns `false` unconditionally and should
    /// not be used.
    #[deprecated]
    pub fn is_subset(&self, _other: Self) -> bool {
        false
    }
}
