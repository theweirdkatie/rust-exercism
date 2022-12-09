use std::collections::HashMap;
pub fn check(candidate: &str) -> bool {
    let mut map = HashMap::new();
    for c in candidate.chars() {
        if c.is_alphabetic() {
            *map.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
        }
    }
    map.len() == 0 || !map.into_values().any(|x| x > 1)
}
