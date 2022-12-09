/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut alphabet = ('A'..='Z').collect::<Vec<char>>();
    for c in sentence.chars() {
        if let Some(i) = alphabet.iter().position(|&x| x == c.to_ascii_uppercase()) {
            alphabet.remove(i);
        }
    }
    alphabet.len() == 0
}