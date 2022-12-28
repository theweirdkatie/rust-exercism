pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let luhn = self.to_string();
        if luhn.len() <= 1 || luhn.chars().any(|x| !x.is_ascii_digit() && !x.is_ascii_whitespace()) {
            false
        } else {
            let sum_nums: u32 = luhn
                .chars()
                .rev()
                .filter_map(|ch| ch.to_digit(10))
                .enumerate()
                .map(|(i, n)| match i % 2 {
                    0 => n,
                    _ if n == 9 => n,
                    _ => (n * 2) % 9,
                })
                .sum();
                sum_nums % 10 == 0
        }
    }
}
