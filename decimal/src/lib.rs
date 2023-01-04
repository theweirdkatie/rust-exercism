use core::num::dec2flt::number;
use std::{cmp::Ordering, io::Read, ops::{Add, Sub, Mul}};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Sign {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Decimal {
    sign: Sign,
    number: Vec<u8>,
    fraction: Vec<u8>,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        println!("From {input}:");
        if let Some((left, right)) = input.split_once(".") {
            if let Some(_) = left.chars().nth(0) {
                let number: Vec<u8> = left.chars().inspect(|ch| println!("{ch}"))
                    .map_while(|ch| ch.to_digit(10))
                    .map(|i| i as u8)
                    .collect();
                let fraction: Vec<u8> = right.chars()
                    .map_while(|ch| ch.to_digit(10))
                    .map(|i| i as u8)
                    .collect();
                if number.len() == left.len() && fraction.len() == right.len() {
                    return Some(Decimal { sign: Sign::Positive, number, fraction, });
                }
            }
        }
        None
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.sign, other.sign) {
            (Sign::Positive, Sign::Negative) => Some(Ordering::Greater),
            (Sign::Negative, Sign::Positive) => Some(Ordering::Less),
            (_,_) => {
                for (s,o) in self.number.iter().zip(other.number.iter()) {
                    if s > o {
                        return Some(Ordering::Greater);
                    } else if s < o {
                        return Some(Ordering::Less);
                    }
                }
                for (s,o) in self.fraction.iter().zip(other.fraction.iter()) {
                    if s > o {
                        return Some(Ordering::Greater);
                    } else if s < o {
                        return Some(Ordering::Less);
                    }
                }
                Some(Ordering::Equal)
            }
        }
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut remainder = 0;
        let number = self.number.iter().zip(rhs.number.iter()).rev().map(|(s,o)| {
            let sum = s+o+remainder;
            if sum > 10 {
                let result = sum % 10;
                remainder = sum-result;
                return result;
            } else {
                return sum;
            }
        }).rev().collect();
        remainder = 0;
        let fraction = self.fraction.iter().zip(rhs.fraction.iter()).rev().map(|(s,o)| {
            let sum = s+o+remainder;
            if sum > 10 {
                let result = sum % 10;
                remainder = sum-result;
                return result;
            } else {
                return sum;
            }
        }).rev().collect();
        Self { sign: self.sign, number, fraction }
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut remainder = 0;
        let number = self.number.iter().zip(rhs.number.iter()).rev().map(|(&s,&o)| {
            let diff: i8 = s as i8 - o as i8 - remainder as i8;
            if let Ok(res) = diff.try_into() {
                return res;
            } else {
                let result: u8 = (diff % 10).try_into().unwrap();
                remainder += 1;
                return result;
            }
        }).rev().collect();
        remainder = 0;
        let fraction = self.fraction.iter().zip(rhs.fraction.iter()).rev().map(|(&s,&o)| {
            let diff: i8 = s as i8 - o as i8 - remainder as i8;
            if let Ok(res) = diff.try_into() {
                return res;
            } else {
                let result: u8 = (diff % 10).try_into().unwrap();
                remainder += 1;
                return result;
            }
        }).rev().collect();
        Self { sign: self.sign, number, fraction }
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut mults = vec![];
        let dec = self.fraction.len() + rhs.fraction.len();
        for s in self.number.iter().chain(self.fraction.iter()).rev() {
            let mut carry = 0;
            let mut product = vec![];
            for r in rhs.number.iter().chain(rhs.fraction.iter()).rev() {
                let mut p = s*r + carry;
                carry = p/10;
                p = p % 10;
                product.push(p);
            }
            product.push(carry);
            product.reverse();
            mults.push(product);
        }

        Self {
            sign: Sign::Positive,
            number: vec![],
            fraction: vec![],
        }
    }
}