pub fn is_armstrong_number(num: u32) -> bool {
    let mut numbers: Vec<u32> = vec![];
    let mut _sum = 0;
    
    // convert the number to a string
    // iterator over characters
    // for each character, convert back to number, push onto number vector
    num.to_string().chars().for_each( |x| numbers.push(x.to_digit(10).unwrap_or_default()));
    
    // iterate over number vector
    // raise each digit to the numbers of digits and add to sum
    numbers.iter().for_each( |x| _sum += x.pow(numbers.len().try_into().unwrap()));
    
    // if sum == original number, is armstrong number
    _sum == num
}
