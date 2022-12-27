pub fn get_diamond(c: char) -> Vec<String> {
    let diff = (u32::from(c.to_ascii_uppercase()) - u32::from('A')) as usize;
    let mut result = vec![];
    for (i, letter) in ('A'..=c.to_ascii_uppercase()).enumerate() {
        result.push(letter.to_string());
        if i > 0 {
            for _ in 0..(i*2 - 1) {
                result[i].push(' ');
            }
            result[i].push(letter);
        }
        for _ in i..diff {
            result[i] = format!(" {} ", result[i]);
        }
    }
    if result.len() > 1 {
        for i in (1..result.len()).rev() {
            result.push(result[i-1].clone());
        }
    }
    result
}
