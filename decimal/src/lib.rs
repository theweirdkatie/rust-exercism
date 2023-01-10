use std::{
    cmp::Ordering,
    ops::{Add, Mul, Sub, Not},
};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Sign {
    Positive,
    Negative,
}

impl Not for Sign {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Decimal {
    sign: Sign,
    digits: Vec<u8>,
    factor: usize,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let sign = if input.starts_with('-') {
            Sign::Negative
        } else {
            Sign::Positive
        };
        let input_stripped: String = input.chars().filter(|&ch| ch.is_numeric() || ch=='.').collect();
        let mut factor = 0;
        let digits = input_stripped.chars()
            .fold(Vec::with_capacity(input.len()), |mut v, c| {
                if c.is_numeric() {
                    v.push(c.to_digit(10).unwrap() as u8);
                } else if c == '.' {
                    factor = input_stripped.len() - v.len() - 1;
                }
                v
            });
        
        Some(Decimal::new(sign, digits, factor))
    }

    fn new(sign: Sign, mut digits: Vec<u8>, mut factor: usize) -> Decimal {
        while digits.ends_with(&[0]) && factor > 0 {
            digits.pop();
            factor -= 1;
        }
        while digits.starts_with(&[0]) && digits.len() > factor+1 {
            digits.remove(0);
        }
        Decimal {sign, digits, factor}
    }

    fn normalize(lhs: Decimal, rhs: Decimal) -> (Decimal, Decimal) {
        let (mut lhs_norm, mut rhs_norm) = match lhs.factor.cmp(&rhs.factor) {
            Ordering::Greater => {
                (lhs.clone(), 
                Decimal { 
                    sign: rhs.sign, 
                    digits: rhs.digits.clone().into_iter().chain(vec![0_u8; lhs.factor - rhs.factor].into_iter()).collect(), 
                    factor: lhs.factor
                    }
                )
            }
            Ordering::Less => {
                (Decimal { 
                    sign: lhs.sign, 
                    digits: lhs.digits.clone().into_iter().chain(vec![0_u8; rhs.factor - lhs.factor].into_iter()).collect(), 
                    factor: rhs.factor
                    },
                rhs.clone())
            }
            _ => (lhs.clone(), rhs.clone()),
        };
        if lhs_norm.digits.len() != rhs_norm.digits.len() {
            lhs_norm.pad(rhs_norm.digits.len().checked_sub(lhs_norm.digits.len()).unwrap_or(0));     
            rhs_norm.pad(lhs_norm.digits.len().checked_sub(rhs_norm.digits.len()).unwrap_or(0));
        }
        (lhs_norm, rhs_norm)
    }

    fn pad(&mut self, num: usize) {
        if num > 0 {
            self.digits = vec![0; num].into_iter().chain(self.digits.clone().into_iter()).collect();
        }
    }

    fn to_flip_sign(&self) -> Self {
        Decimal::new(!self.sign, self.digits.clone(), self.factor)
    }

    fn to_positive(&self) -> Self {
        Decimal::new(Sign::Positive, self.digits.clone(), self.factor)
    }

}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.sign, other.sign) {
            (Sign::Positive, Sign::Negative) => Some(Ordering::Greater),
            (Sign::Negative, Sign::Positive) => Some(Ordering::Less),
            (some_sign, _) => {
                let (self_norm, other_norm): (Decimal, Decimal) = Decimal::normalize(self.clone(), other.clone());
                for (s,o) in self_norm.digits.iter().zip(other_norm.digits.iter()) {
                    if s>o && some_sign==Sign::Positive || s<o && some_sign==Sign::Negative {
                        return Some(Ordering::Greater);
                    } else if s>o && some_sign==Sign::Negative || s<o && some_sign==Sign::Positive {
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
        if self.sign != rhs.sign {
            if self.sign == Sign::Negative {
                rhs - self.to_flip_sign()
            } else {
                self - rhs.to_flip_sign()
            }
        } else {
            let (self_norm, rhs_norm): (Decimal, Decimal) = Decimal::normalize(self.clone(), rhs.clone());
            
            let mut remainder = 0;
            let mut digits: Vec<u8> = self_norm.digits.iter()
                    .zip(rhs_norm.digits.iter())
                    .rev()
                    .map(|(s, o)| {
                        let sum = s + o + remainder;
                        if sum >= 10 {
                            let result = sum % 10;
                            remainder = (sum - result)/10;
                            return result;
                        } else {
                            remainder = 0;
                            return sum;
                        }
                    })
                    .collect();
            digits.reverse();

            Decimal::new(self.sign, digits, self_norm.factor)
        }
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        if rhs.sign == Sign::Negative {
            return self + rhs.to_positive();
        } else if self.sign == Sign::Negative {
            return (self.to_positive() + rhs).to_flip_sign();
        }
        let (lhs_norm, rhs_norm): (Decimal, Decimal);
        if self >= rhs {
            (lhs_norm, rhs_norm) = Decimal::normalize(self.clone(), rhs.clone());
        } else {
            (lhs_norm, rhs_norm) = Decimal::normalize(rhs.clone(), self.clone());
        }
        
        let mut carry = 0;
        let mut digits: Vec<u8> = lhs_norm.digits.iter()
            .zip(rhs_norm.digits.iter())
            .rev()
            .map(|(&s, &o)| {
                let diff: i8 = s as i8 - o as i8 - carry as i8;
                if diff < 0 {
                    let result = (s+10) - o - carry;
                    carry = 1;
                    return result;
                } else {
                    carry = 0;
                    return diff as u8;
                }
            })
            .collect();
        digits.reverse();

        let sign = match self.partial_cmp(&rhs) {
            Some(Ordering::Less) => !self.sign,
            _ => self.sign,
        };
        
        Decimal::new(sign, digits, lhs_norm.factor)

    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut mults = vec![];
        for s in self.digits.iter().rev() {
            let mut carry = 0;
            let mut product = vec![];
            for r in rhs.digits.iter().rev() {
                let mut p = s * r + carry;
                carry = p / 10;
                p = p % 10;
                product.push(p);
            }
            if carry != 0 {
                product.push(carry);
            }
            product.reverse();
            mults.push(product);
        }

        let mut digits: Vec<u8> = vec![];
        for (exp, prod) in mults.iter().enumerate() {
            let mut line = prod.clone();
            line.append(&mut vec![0; exp]);
            while line.len() > digits.len() {
                digits.insert(0, 0);
            }
            let mut carry = 0;
            for (i,val) in line.iter().enumerate() {
                digits[i] += val + carry;
                if digits[i] > 10 {
                    carry = digits[i]/10;
                    digits[i] = digits[i] % 10;
                }
            }
        }
        
        let sign = if self.sign != rhs.sign { Sign::Negative } else { Sign::Positive };
        
        Decimal::new(sign, digits, self.factor + rhs.factor)

    }
}
