pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if self.0.len() <= 1 || self.0.chars().any(|x| !x.is_ascii_digit() && !x.is_ascii_whitespace()) {
            false
        } else {
            Self::sum_nums(self.0.chars().collect()) % 10 == 0
        }
    }
    pub fn sum_nums(nums: Vec<char>) -> u32 {
        nums.iter()
            .rev()
            .filter_map(|ch| ch.to_digit(10))
            .enumerate()
            .map(|(i, n)| match i % 2 {
                0 => n,
                _ if n == 9 => n,
                _ => (n * 2) % 9,
            })
            .sum::<u32>()
    }
}

impl<T> From<T> for Luhn where T: ToString {
    fn from(input: T) -> Self {
        Self(input.to_string())
    }
}
