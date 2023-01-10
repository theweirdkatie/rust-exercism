use std::fmt::{Display, Formatter, Result};

pub struct Roman(Vec<String>);

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        write!(_f, "{}", self.0.concat())
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let mut numerals = vec![];
        let mut remainder = num;
        for (num, chrs) in dict() {
            while num <= remainder {
                remainder = remainder.wrapping_sub(num);
                numerals.push(chrs.clone());
            }
        }
        Self(numerals)
    }
}

fn dict() -> Vec<(u32, String)> {
    vec![
        (1000, "M".to_string()),
        (900, "CM".to_string()),
        (500, "D".to_string()),
        (400, "CD".to_string()),
        (100, "C".to_string()),
        (90, "XC".to_string()),
        (50, "L".to_string()),
        (40, "XL".to_string()),
        (10, "X".to_string()),
        (9, "IX".to_string()),
        (5, "V".to_string()),
        (4, "IV".to_string()),
        (1, "I".to_string()),
    ]
}
