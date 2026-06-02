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
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Set<T>
where
    T: PartialEq + Clone,
{
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn insert(&mut self, value: T) {
        if !self.contains(&value) {
            self.elements.push(value);
        }
    }

    pub fn remove(&mut self, value: &T) -> bool {
        if let Some(index) = self.elements.iter().position(|x| x == value) {
            self.elements.remove(index);
            return true;
        }

        false
    }

    pub fn contains(&self, value: &T) -> bool {
        self.elements.contains(value)
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut result = self.clone();

        for value in &other.elements {
            result.insert(value.clone());
        }

        result
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let mut result = Self::new();

        for value in &self.elements {
            if other.contains(value) {
                result.insert(value.clone());
            }
        }

        result
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut result = Self::new();

        for value in &self.elements {
            if !other.contains(value) {
                result.insert(value.clone());
            }
        }

        result
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.elements.iter().all(|x| other.contains(x))
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

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
