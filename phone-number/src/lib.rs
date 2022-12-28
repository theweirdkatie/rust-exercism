pub fn number(user_number: &str) -> Option<String> {
    let number = user_number.chars().filter(|ch| ch.is_digit(10)).collect::<String>();
    match number.len() {
        11 => {
            if &number[0..1] == "1"  && is_valid(&number[1..]) {
                return Some(number[1..].to_string());
            } else {
                return None;
            }
        }
        10 => if is_valid(&number) { Some(number) } else { None },
        _ => None,
    }
}

pub fn is_valid(phone_number: &str) -> bool {
    ("2"..="9").contains(&&phone_number[0..1]) && ("2"..="9").contains(&&phone_number[3..4])
}
