use std::cmp::Ordering;
#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 { return None; }
    let sum = (1..num).fold(0, |acc, n| if num % n == 0 {acc + n} else {acc + 0});
    match sum.cmp(&num) {
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Greater => Some(Classification::Abundant),
        Ordering::Less => Some(Classification::Deficient),
    }
}