use std::collections::HashMap;

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    for word in words.split(['\n', ' ', ',']) {
        let len = word.len();
        let ent = word.char_indices()
            .filter(|(i, c)| c.is_ascii_alphanumeric() || (*c == '\'' && *i != 0 && i + 1 != len))
            .map(|(_, x)| x.to_ascii_lowercase())
            .collect::<String>();
        if ent.len() > 0 {
            *map.entry(ent).or_insert(0) += 1;
        }
    }
    map
}
