#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    values: Vec<T>,
}

impl<T: Copy + PartialEq + Ord> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        let mut result: CustomSet<T> = CustomSet { values: vec![] };
        for _in in _input {
            result.add(*_in);
        }
        result
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.values.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        self.values.push(_element);
        self.values.dedup();
        self.values.sort();
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        _other.values.iter()
            .filter(|x| self.contains(&x))
            .map(|y| *y)
            .collect::<Vec<T>>() == self.values
    }

    pub fn is_empty(&self) -> bool {
        self.values.len() == 0
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        self.values.iter().all(|val| !_other.contains(val))
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        Self {
            values: self.values.iter().filter(|x| _other.contains(x)).map(|y| *y).collect(),
        }
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        Self {
            values: self.values.iter().filter(|x| !_other.values.contains(x)).map(|y| *y).collect()
        }
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        let values = [self.values.clone().as_slice(), _other.values.clone().as_slice()].concat();
        Self::new(&values)
    }
}