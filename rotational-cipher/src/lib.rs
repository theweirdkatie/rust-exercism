pub fn rotate(input: &str, key: i8) -> String {
    let mut cipher_alphabet = ('a'..='z').collect::<Vec<char>>();
    if key > 0 {
        cipher_alphabet.rotate_left(key.abs() as usize);
    } else {
        cipher_alphabet.rotate_right(key.abs() as usize);
    }
    input
        .chars()
        .map(|ch| {
            if let Some(i) = ('a'..='z').position(|c| c == ch) {
                cipher_alphabet[i]
            } else if let Some(j) = ('A'..='Z').position(|c| c == ch) {
                cipher_alphabet[j].to_ascii_uppercase()
            } else {
                ch
            }
        })
        .collect::<String>()
}
