/// A simple mathematical set.
///
/// This collection stores unique values and provides common
/// set operations such as union, intersection, and difference.
///
/// # Type Parameters
///
/// * `T` - Element type stored in the set.
///
/// # Examples
///
/// ```rust
/// use mathrc::Set;
///
/// let mut set = Set::new();
///
/// set.insert(1);
/// set.insert(2);
/// set.insert(2);
///
/// assert_eq!(set.len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct Set<T>
where
    T: PartialEq + Clone,
{
    elements: Vec<T>,
}

impl<T> Default for Set<T>
where
    T: PartialEq + Clone,
{
    /// Creates an empty set.
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Set<T>
where
    T: PartialEq + Clone,
{
    /// Creates an empty set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Set;
    ///
    /// let set: Set<i32> = Set::new();
    ///
    /// assert!(set.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    /// Inserts a value into the set.
    ///
    /// If the value already exists, no action is taken.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Set;
    ///
    /// let mut set = Set::new();
    ///
    /// set.insert(1);
    /// set.insert(1);
    ///
    /// assert_eq!(set.len(), 1);
    /// ```
    pub fn insert(&mut self, value: T) {
        if !self.contains(&value) {
            self.elements.push(value);
        }
    }

    /// Removes a value from the set.
    ///
    /// Returns `true` if the value was present.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Set;
    ///
    /// let mut set = Set::new();
    /// set.insert(1);
    ///
    /// assert!(set.remove(&1));
    /// ```
    pub fn remove(&mut self, value: &T) -> bool {
        if let Some(index) = self.elements.iter().position(|x| x == value) {
            self.elements.remove(index);
            return true;
        }

        false
    }

    /// Returns `true` if the set contains the specified value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Set;
    ///
    /// let mut set = Set::new();
    /// set.insert(42);
    ///
    /// assert!(set.contains(&42));
    /// ```
    pub fn contains(&self, value: &T) -> bool {
        self.elements.contains(value)
    }

    /// Returns the union of two sets.
    ///
    /// The resulting set contains all unique elements
    /// from both sets.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Set;
    ///
    /// let mut a = Set::new();
    /// a.insert(1);
    ///
    /// let mut b = Set::new();
    /// b.insert(2);
    ///
    /// let c = a.union(&b);
    ///
    /// assert_eq!(c.len(), 2);
    /// ```
    pub fn union(&self, other: &Self) -> Self {
        let mut result = self.clone();

        for value in &other.elements {
            result.insert(value.clone());
        }

        result
    }

    /// Returns the intersection of two sets.
    ///
    /// The resulting set contains only elements
    /// that exist in both sets.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Set;
    ///
    /// let mut a = Set::new();
    /// a.insert(1);
    /// a.insert(2);
    ///
    /// let mut b = Set::new();
    /// b.insert(2);
    /// b.insert(3);
    ///
    /// let c = a.intersection(&b);
    ///
    /// assert_eq!(c.len(), 1);
    /// ```
    pub fn intersection(&self, other: &Self) -> Self {
        let mut result = Self::new();

        for value in &self.elements {
            if other.contains(value) {
                result.insert(value.clone());
            }
        }

        result
    }

    /// Returns the difference of two sets.
    ///
    /// The resulting set contains elements that are
    /// present in `self` but not in `other`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Set;
    ///
    /// let mut a = Set::new();
    /// a.insert(1);
    /// a.insert(2);
    ///
    /// let mut b = Set::new();
    /// b.insert(2);
    ///
    /// let c = a.difference(&b);
    ///
    /// assert!(c.contains(&1));
    /// ```
    pub fn difference(&self, other: &Self) -> Self {
        let mut result = Self::new();

        for value in &self.elements {
            if !other.contains(value) {
                result.insert(value.clone());
            }
        }

        result
    }

    /// Returns `true` if this set is a subset of `other`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Set;
    ///
    /// let mut a = Set::new();
    /// a.insert(1);
    ///
    /// let mut b = Set::new();
    /// b.insert(1);
    /// b.insert(2);
    ///
    /// assert!(a.is_subset(&b));
    /// ```
    pub fn is_subset(&self, other: &Self) -> bool {
        self.elements.iter().all(|x| other.contains(x))
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns `true` if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Removes all elements from the set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Set;
    ///
    /// let mut set = Set::new();
    /// set.insert(1);
    ///
    /// set.clear();
    ///
    /// assert!(set.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    /// Returns an iterator over the set elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathrc::Set;
    ///
    /// let mut set = Set::new();
    /// set.insert(1);
    ///
    /// for value in set.iter() {
    ///     println!("{}", value);
    /// }
    /// ```
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.elements.iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set() {
        let mut a = Set::new();
        a.insert(1);
        a.insert(2);
        a.insert(3);

        let mut b = Set::new();
        b.insert(3);
        b.insert(4);
        b.insert(5);

        assert!(a.contains(&1));
        assert!(!a.contains(&4));

        assert_eq!(a.union(&b).len(), 5);
        assert_eq!(a.intersection(&b).len(), 1);
        assert_eq!(a.difference(&b).len(), 2);

        assert!(Set::<i32>::new().is_subset(&a));
    }
}
