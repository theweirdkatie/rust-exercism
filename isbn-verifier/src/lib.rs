/// ISBN pattern:
/// (10x_1 + 9x_2 + 8x_3 + 7x_4 + 6x_5 + 5x_6 + 4x_7 + 3x_8 + 2x_9 + 1x_10) mod 11 == 0
/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let valid_characters: String = isbn
        .replace("-", "")
        .chars()
        .filter(|&x| x.is_digit(10) || x == 'X')
        .collect();

    valid_characters.chars()
                    .enumerate()
                    .fold(0, |acc, (i, num)| {
                        acc + num.to_digit(10)
                            .unwrap_or_else(|| if i == 9 { 10 } else { 0 })
                            * (i as u32 + 1)
                    }) % 11 == 0
                    && valid_characters.len() == 10
}
