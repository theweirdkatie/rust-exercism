/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // filter white spaces
    let mut _numbers: Vec<char> = code.chars().filter( |x| !x.is_ascii_whitespace() ).collect();
    
    // check for invalid characters
    if let Some(_) = _numbers.iter().find( |x| !x.is_ascii_digit() ) {
        return false;
    }

    // limit to more than 1 number
    if _numbers.len() <= 1 { 
        false 
    } else { 
        _numbers.iter()
                .rev()
                .filter_map(|ch| ch.to_digit(10))
                .enumerate()
                .map(|(i, n)| match i % 2 {
                    0 => n,
                    _ if n == 9 => n,
                    _ => (n * 2) % 9,
                })
                .sum::<u32>() % 10 == 0
    }
}