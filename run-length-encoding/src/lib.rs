use std::thread::current;

pub fn encode(source: &str) -> String {
    let mut encoded_string = String::new();
    let mut count: u32 = 1;
    for (i, current_char) in source.chars().enumerate() {
        if let Some(next_char) = source.chars().nth(i + 1) {
            if current_char == next_char {
                count += 1;
            } else {
                if count > 1 {
                    encoded_string.push_str(&count.to_string());
                }
                encoded_string.push(current_char);
                count = 1;
            }
        } else {
            if count > 1 {
                encoded_string.push_str(&count.to_string());
            }
            encoded_string.push(current_char);
        }
    }
    encoded_string
}

pub fn decode(source: &str) -> String {
    source
        .split(|c: char| !c.is_digit(10))
        .map(|n_str| n_str.parse::<usize>().unwrap_or(1))
        .zip(source.matches(|c: char| !c.is_digit(10)))
        .map(|(num, c)| c.repeat(num))
        .collect()
}