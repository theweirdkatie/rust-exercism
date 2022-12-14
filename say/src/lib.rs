pub fn encode(n: u64) -> String {
    if n < 20 {
        let ones_and_teens = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", 
                "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
                ];
        String::from(ones_and_teens[n as usize])
    } else if n < 100 {
        let tens = vec![
            "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
        ];
        let mut encoded_number = String::from(tens[(n/10 - 2) as usize]);
        if n % 10 != 0 {
            encoded_number.push('-');
            encoded_number.push_str(encode(n % 10).as_str());
        }
        encoded_number
    } else if n < 1000{
        let mut encoded_number = String::from(encode(n/100).as_str());
        encoded_number.push_str(" hundred");
        if n % 100 != 0 {
            encoded_number.push(' ');
            encoded_number.push_str(encode(n % 100).as_str());
        }
        encoded_number
    } else {
        let large_numbers = vec!["thousand", "million", "billion", "trillion", "quadrillion", "quintillion"];
        let num_size = (n as f32).log(1000.0).trunc() as u32;
        if num_size as usize > large_numbers.len() { panic!("Can't parse numbers that large!");}
        let mut encoded_number = String::from(encode(n/(1000_u64.pow(num_size))).as_str());
        encoded_number.push_str(" ");
        encoded_number.push_str(large_numbers[num_size as usize -1]);
        if n % (1000_u64.pow(num_size)) != 0 {
            encoded_number.push(' ');
            encoded_number.push_str(encode(n % (1000_u64.pow(num_size))).as_str());
        }
        encoded_number
    }
}
