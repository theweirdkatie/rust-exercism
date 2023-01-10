use std::ops::Rem;

pub struct Matcher<T> {
    _fn: Box<dyn Fn(T) -> bool>,

    substitution: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(_matcher: F, _subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: ToString,
    {
        Self {
            _fn: Box::new(_matcher),
            substitution: _subs.to_string(),
        }
    }

    pub fn substitute(&self, val: T) -> Option<String> {
        if (self._fn)(val) {
            Some(self.substitution.to_string())
        } else {
            None
        }
    }
}

pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T> Fizzy<T>
where
    T: Clone + ToString,
{
    pub fn new() -> Self {
        Self { matchers: vec![] }
    }

    #[must_use]
    pub fn add_matcher(mut self, _matcher: Matcher<T>) -> Self {
        self.matchers.push(_matcher);
        self
    }

    pub fn apply<I>(self, _iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        _iter.map(move |item| {
            let val = self.matchers.iter()
                .flat_map(|matcher| matcher.substitute(item.clone()))
                .collect::<Vec<String>>()
                .concat();
            if val.is_empty() {
                item.to_string()
            } else {
                val
            }
        })
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Clone + ToString + Rem<T, Output = T> + PartialEq + From<u8>,
{
    Fizzy::<T>::new()
        .add_matcher(Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"))
}
