pub fn encrypt(input: &str) -> String {
    if input.len() < 1 {
        return "".to_string();
    }
    let normalized = input
        .to_ascii_lowercase()
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .map(|ch| ch.to_string())
        .collect::<Vec<String>>();
    let cols = (normalized.len() as f64).sqrt().ceil() as usize;
    let square = normalized
        .chunks(cols)
        .map(|slice| {
            let mut stri = slice.join("");
            while stri.len() < cols {
                stri.push(' ');
            }
            stri
        })
        .collect::<Vec<String>>();
    let mut result = vec![];
    for row in 0..square.len() {
        for (i, char) in square[row].char_indices() {
            if row == 0 {
                result.push(char.to_string());
            } else {
                result[i].push(char);
            }
        }
    }
    result.join(" ")
}
