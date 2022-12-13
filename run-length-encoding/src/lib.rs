use std::thread::current;

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
    let organized_vec = organize_vec(source);
    let mut decoded_string = String::new();
    for entry in organized_vec.iter() {
        if entry.len() > 1 {
            if let Ok(count) = entry[..(entry.len()-1)].parse() {
                for _ in 0..count {
                    decoded_string.push_str(&entry[entry.len()-1..]);
                }
            }
        } else {
            decoded_string.push_str(&entry[..entry.len()]);
        }
    }
    decoded_string
}

pub fn organize_vec(source: &str) -> Vec<String> {
    let mut iter = source.chars();
    let mut result = vec![];
    let mut count = 1;
    result.push("".to_string());
    while let Some(current_char) = iter.next() {
        let index = result.len()-1;
        if current_char.is_ascii_digit() {
            count += 1;
        } else {
            count = 1;
        }
        result[index].push(current_char);
        if count == 1 {result.push("".to_string());}
    }
    result
}