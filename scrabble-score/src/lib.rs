use std::collections::HashMap;

const SCRABBLE_VALUES: [(&str, usize); 7] = [
    ("AEIOULNRST", 1),
    ("DG", 2),
    ("BCMP", 3),
    ("FHVWY", 4),
    ("K", 5),
    ("JX", 8),
    ("QZ", 10),
];

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let scrabble_letter_map = create_hashmap(&SCRABBLE_VALUES);
    word.chars()
        .map(|x| {
            *scrabble_letter_map
                .get(&(x.to_ascii_uppercase()))
                .unwrap_or(&0) as u64
        })
        .sum()
}

pub fn create_hashmap(input: &[(&str, usize)]) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    for (entry, points) in input {
        for ch in entry.chars() {
            result.insert(ch, *points);
        }
    }
    result
}
