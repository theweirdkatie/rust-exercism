pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(word) => list
            .windows(2)
            .map(|_str| format!("For want of a {} the {} was lost.\n", _str[0], _str[1]))
            .collect::<String>() + &format!("And all for the want of a {}.", word)
    }
}