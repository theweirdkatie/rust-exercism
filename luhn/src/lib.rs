/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // filter white spaces
    let mut _numbers: Vec<char> = code.chars().filter( |x| !x.is_ascii_whitespace() ).collect();
    
    // check for invalid characters
    if let Some(_) = _numbers.iter().find( |x| !x.is_ascii_digit() ) {
        return false;
    }

    // limit to more than 1 number
    if _numbers.len() <= 1 { 
        false 
    } else { 
        let _new_numbers: Vec<char> = double_second(_numbers);
        sum_nums(_new_numbers) % 10 == 0
    }
}

pub fn double_second(number: Vec<char>) -> Vec<char> {
    let mut result: Vec<char> = vec![];
    for (i, entry) in number.iter().rev().enumerate() {
        // index is 0 based, if index is odd then second digit => double
        if i % 2 != 0 {
            let mut new_entry = entry.to_digit(10).unwrap_or_default() * 2;
            if new_entry > 9 { new_entry -= 9 } else {}; // reduce overflow
            result.push(char::from_digit(new_entry, 10).unwrap_or_default());
        } else {
            result.push(*entry);
        }
    }
    result
}

pub fn sum_nums(nums: Vec<char>) -> u32 {
    let mut _sum: u32 = 0;
    for num_char in nums {
        _sum += num_char.to_digit(10).unwrap_or_default();
    }
    _sum
}
