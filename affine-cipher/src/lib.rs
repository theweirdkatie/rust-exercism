/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    // E(x) = (ax + b) mod m where m = 26, x = letter index, a & b are keys
    if !check_coprime(a, 26) { return Err(AffineCipherError::NotCoprime(a)); }
    let mut result: Vec<String> = vec!["".to_string()];
    for ch in plaintext.chars().filter(|x| x.is_alphanumeric()) {
        let mut new_char = ch;
        if ch.is_alphabetic() {
            let index = ((a * transpose(ch) as i32 + b) % 26) as usize;
            new_char = get_char(index);
        }
        let ln = result.len();
        if let Some(st) = result.get_mut(ln - 1) {
            if st.len() < 5 {
                st.push(new_char);
            } else {
                result.push(format!("{}", new_char));
            }
        }
    }
    Ok(result.join(" "))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    // D(y) = a^-1(y - b) mod m where m = 26, x = letter index, a & b are keys
    if !check_coprime(a, 26) { return Err(AffineCipherError::NotCoprime(a)); }
    let mut result: Vec<String> = vec!["".to_string()];
    for ch in ciphertext.chars().filter(|x| x.is_alphanumeric()) {
        let mut new_char = ch;
        if ch.is_alphabetic() {
            let index: usize = (1..26).find(|x| ((a % 26) * (x % 26)) % 26 == 1 ).unwrap_or(0) as usize;
            new_char = get_char(index);
        }
        let ln = result.len();
        if let Some(st) = result.get_mut(ln - 1) {
            if st.len() < 5 {
                st.push(new_char);
            } else {
                result.push(format!("{}", new_char));
            }
        }
    }
    Ok(result.join(" "))
}

pub fn transpose(ch: char) -> usize {
    ('a'..='z')
        .position(|c| c == ch.to_ascii_lowercase())
        .unwrap()
}

pub fn get_char(i: usize) -> char {
    ('a'..='z').nth(i).unwrap()
}

pub fn check_coprime(mut a: i32, mut b: i32) -> bool {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a == 1
}