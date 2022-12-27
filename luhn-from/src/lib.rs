pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if self.0.len() <= 1 || self.0.chars().any(|x| !x.is_ascii_digit() && !x.is_ascii_whitespace()) {
            println!("{}", self.0.len());
            false
        } else {
            Self::sum_nums(self.0.chars().filter( |x| !x.is_ascii_whitespace() ).collect()) % 10 == 0
        }
    }
    pub fn sum_nums(nums: Vec<char>) -> u32 {
        Self::double_second(nums).iter().map(|x| x.to_digit(10).unwrap_or_default()).sum::<u32>()
    }
    fn double_second(number: Vec<char>) -> Vec<char> {
        number.iter().rev().enumerate().map(|(i, &en)|{
            if i % 2 != 0 {
                let mut new_entry = en.to_digit(10).unwrap_or_default() * 2;
                if new_entry > 9 { new_entry -= 9 } else {}; // reduce overflow
                char::from_digit(new_entry, 10).unwrap_or_default()
            } else {
                en
            }
        }).collect::<Vec<char>>()
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn where T: ToString {
    fn from(input: T) -> Self {
        Self(input.to_string())
    }
}
