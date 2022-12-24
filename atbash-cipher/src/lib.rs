use std::char::from_u32;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter(|x| x.is_alphanumeric())
        .map(|ch| {
            if let Some(i) = ('a'..='z').position(|l| l==ch.to_ascii_lowercase()) {
                ('a'..='z').rev().nth(i).unwrap_or_default()
            } else {
                ch
            }
        })
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|st| st.iter().collect::<String>() + " ")
        .collect::<String>().trim().to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|x| x.is_alphanumeric())
        .map(|ch| {
            if let Some(i) = ('a'..='z').rev().position(|l| l==ch.to_ascii_lowercase()) {
                ('a'..='z').nth(i).unwrap()
            } else {
                ch
            }
        })
        .collect::<String>()
}
