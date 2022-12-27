#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    } else if string_digits.len() < span {
        return Err(Error::SpanTooLong);
    }
    let mut largest_product: u64 = 0;
    let mut digits = vec![];
    for ch in string_digits.chars() {
        if let Some(num) = ch.to_digit(10) {
            digits.push(num as u64);
        } else {
            return Err(Error::InvalidDigit(ch));
        }
    }
    digits.windows(span).for_each(|sp| if sp.iter().product::<u64>() > largest_product { largest_product = sp.iter().product() });
    Ok(largest_product)
}
