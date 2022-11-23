pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split_ascii_whitespace()
        .map(|x| x.get(0..1).unwrap())
        .collect::<String>()
        .to_uppercase()
}
