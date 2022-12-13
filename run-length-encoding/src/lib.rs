pub fn encode(source: &str) -> String {
    let mut encoded_string = String::new();
    let mut count: u32 = 1;
    for (i, current_char) in source.chars().enumerate() {
        if let Some(next_char) = source.chars().nth(i+1) {
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
    let mut decoded_string = String::new();
    let mut decode_iter = source.chars().enumerate();
    while let Some((i, current_char)) = decode_iter.next() {
        if let Some(count) = current_char.to_digit(10) {
            for _ in 0..count {
                decoded_string.push(source.chars().nth(i+1).unwrap());
            }
            decode_iter.next();
        } else {
            decoded_string.push(current_char);
        }
    }
    decoded_string
}
