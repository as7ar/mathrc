#[derive(Debug, Clone, )]
pub struct Set<T> where T: PartialEq + Clone,  {
    elements: Vec<T>
}

impl<T> Set<T> where T: PartialEq + Clone,  {
    pub fn new() -> Self {
        Self { elements: Vec::new() }
    }

    pub fn insert(&mut self, n: T) {
        if !self.elements.contains(&n) {
            self.elements.push(n)
        }
    }

    pub fn remove(&mut self, n: T) -> bool {
        if let Some(index) = self.elements.iter().position(
            |x| *x == n
        ) {
            self.elements.remove(index);
            return true;
        }
        false
    }

    pub fn contains(&self, n: T) -> bool {
        self.elements.contains(&n)
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut result = self.clone();

        for x in &other.elements {
            result.insert(x.clone());
        }
        result
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let mut result = Self::new();

        for x in &self.elements {
            if other.contains(x.clone()) {
                result.insert(x.clone());
            }
        }
        result
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut result = Self::new();

        for x in &self.elements {
            if !other.contains(x.clone()) {
                result.insert(x.clone());
            }
        }
        result
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.elements.iter().all(|x| other.contains(x.clone()))
    }
}