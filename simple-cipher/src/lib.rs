use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.chars().any(|x| x.is_ascii_uppercase() || x.is_numeric()) || key.is_empty() {
        None
    } else {
        Some(s.chars()
            .zip(key.chars().cycle())
            .map(|(ch, k)| {
                char::from((ch as u8 - b'a' + (k as u8 - b'a' + 26) as u8) % 26 + b'a')
            })
            .collect::<String>(),
        )
    }
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.chars().any(|x| x.is_ascii_uppercase() || x.is_numeric()) || key.is_empty() {
        None
    } else {
        Some(s.chars()
            .zip(key.chars().cycle())
            .map(|(ch, k)| {
                char::from((ch as u8 - b'a' + (26 - (k as u8 - b'a')) as u8) % 26 + b'a')
            })
            .collect::<String>(),
        )
    }
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let key = (0..100).map(|_| char::from(rng.gen_range(b'a'..=b'z'))).collect::<String>();
    (key.clone(), encode(&key, s).unwrap_or_default())
}
