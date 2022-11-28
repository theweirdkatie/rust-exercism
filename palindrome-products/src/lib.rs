/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let num: Vec<char> = value.to_string().chars().collect();
        let sorted_num: Vec<char> = num.iter().copied().rev().collect();
        if num == sorted_num {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut result: Vec<Palindrome> = vec![];
    for a in min..=max {
        for b in min..=max {
            if b>=a {
                if let Some(val) = Palindrome::new(b*a) {
                    result.push(val);
                }
            } else {};
        }
    }
    if result.len() > 0 {
        result.sort_by(|Palindrome(a), Palindrome(b)| a.cmp(&b));
        Some((result[0], result.pop().unwrap_or_default()))
    } else {
        None
    }
}
