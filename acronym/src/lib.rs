pub fn abbreviate(phrase: &str) -> String {
    let all_words: Vec<&str> = phrase.split(&['-', ' ']).collect::<Vec<&str>>();
    let mut result = String::new();
    for word in all_words {
        result.push_str(
            &word
                    .chars()
                    .enumerate()
                    .filter(|(i, char)| (*i==0 || char.is_uppercase()) && char.is_alphabetic())
                    .map(|(_,c)| c)
                    .collect::<String>()
                );
    }
    
    result.to_uppercase()
}
