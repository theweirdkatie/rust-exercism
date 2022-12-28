pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| match c {
            'a'..='z' => char::from((c as u8 - b'a' + (key + 26) as u8) % 26 + b'a'),
            'A'..='Z' => char::from((c as u8 - b'A' + (key + 26) as u8) % 26 + b'A'),
            _ => c,
        })
        .collect()
}