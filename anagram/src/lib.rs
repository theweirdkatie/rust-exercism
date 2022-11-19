use std::collections::HashSet;

pub fn is_anagram(word: &str, anagram: &str) -> bool {
    let mut word_vec = word.to_lowercase().chars().collect::<Vec<_>>();
    let mut anagram_vec = anagram.to_lowercase().chars().collect::<Vec<_>>();

    if word_vec==anagram_vec {return false};

    word_vec.sort_unstable();
    anagram_vec.sort_unstable();

    if word_vec==anagram_vec { return true } else { return false };
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();

    for anagram in possible_anagrams {
        if is_anagram(word, anagram) { 
            set.insert(*anagram);
        } else {
        };
    };

    set
}
