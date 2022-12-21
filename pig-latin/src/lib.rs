const VOWELS: [char; 5] = ['A', 'E', 'I', 'O', 'U'];

pub fn translate(input: &str) -> String {
    let mut result = String::new();
    for word in input.split_ascii_whitespace() {
        if VOWELS.contains(&word.chars().nth(0).unwrap().to_ascii_uppercase())
            || &word[0..=1] == "xr"
            || &word[0..=1] == "yt"
        {
            result = format!("{} {}ay", result, word);
        } else if &word[1..=2] != "qu" {
            let mut vowel_index: usize = 0;
            if &word[0..=1] == "qu" {
                vowel_index = 2;
            } else {
                vowel_index = word
                    .chars()
                    .enumerate()
                    .position(|(i, x)| {
                        (VOWELS.contains(&x.to_ascii_uppercase()) || x.to_ascii_uppercase() == 'Y')
                            && i > 0
                    })
                    .unwrap_or(1);
            }
            result = format!("{} {}{}ay", result, &word[vowel_index..], &word[..vowel_index]);
        } else {
            result = format!("{} {}{}ay", result, &word[3..], &word[..3])
        }
    }
    return result.trim().to_string();
}