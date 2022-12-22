pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len==0 {
        return vec!["".to_string(); digits.len()+1];
    }
        digits.char_indices().map_while(|(i, _)| digits.get(i..i+len)).map(|sl| sl.to_string()).collect::<Vec<String>>()
}
