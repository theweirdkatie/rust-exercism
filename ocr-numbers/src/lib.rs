// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let vec_input = input.split('\n').collect::<Vec<&str>>();
    if let Err(not_valid_error) = check_valid(vec_input.clone()) {
        return Err(not_valid_error);
    } else if has_invalid_chars(vec_input.clone()) {
        return Ok("?".to_string());
    }

    let mut result = String::new();
    for num in 0..=(vec_input.len() % 4) {
        println!("num {num}/{}", vec_input.len() % 4);
        result = result + &get_num(vec_input[num*4..(num*4)+4].to_vec());
    }

    Ok(result)
}

pub fn check_valid(input: Vec<&str>) -> Result<(), Error> {
    let row_count = input.len();
    let col_count = input.iter().map(|x| x.len()).sum::<usize>()/4;
    if row_count % 4 != 0 {
        return Err(Error::InvalidRowCount(row_count));
    } else if col_count % 3 != 0 {
        return Err(Error::InvalidColumnCount(col_count));
    }
    Ok(())
}

pub fn has_invalid_chars(input: Vec<&str>) -> bool {
    input.iter().inspect(|x| println!("checking -{x}-")).any(|&x| !["| |", "   ", "  |", " _ ", "|_ ", " _|", "|_|"].contains(&x))
}

pub fn get_num(number: Vec<&str>) -> String {
    let mut possible_numbers = (0..10).collect::<Vec<usize>>();
    for (i, row) in number.iter().enumerate() {
            println!("checking -{row}- @{i}");
            match *row {
                "   " => if i < 3 {possible_numbers.retain(|x| [1,4].contains(x))},
                " _ " => possible_numbers.retain(|x| ![1,4].contains(x)),
                "  |" => possible_numbers.retain(|x| [1,4,7].contains(x)),
                "|_|" => { 
                            if i < 2 { 
                                possible_numbers.retain(|x| [4,8,9].contains(x)); 
                            } else {
                                possible_numbers.retain(|x| [6,8,0].contains(x));
                            }
                        }
                " _|" => { 
                    if i < 2 {
                        possible_numbers.retain(|x| [2,3].contains(x)); 
                    } else {
                        possible_numbers.retain(|x| [3,5,9].contains(x));
                    }
                }
                "|_ " => possible_numbers.retain(|x| [2,5,6].contains(x)),
                "| |" => return "0".to_string(),
                _ => panic!("ruh roh"),

                // (1, "|  ") => possible_numbers.retain(|x| [4,6,8,9,0].contains(x)),
                // (1, "  |") => possible_numbers.retain(|x| [4,6,8,9,0].contains(x)),
                // (1, "| |") => possible_numbers.retain(|x| [4,6,8,9,0].contains(x)),
                // (2, "|  ") => possible_numbers.retain(|x| [4,6,8,9,0].contains(x)),
                // (2, "  |") => possible_numbers.retain(|x| [4,6,8,9,0].contains(x)),
                // (2, "| |") => possible_numbers.retain(|x| [4,6,8,9,0].contains(x)),
                // (3, "  |") => possible_numbers.retain(|x| [4,6,8,9,0].contains(x)),
                // (3, "|_|") => possible_numbers.retain(|x| [4,6,8,9,0].contains(x)),
                // (_,_) => {},
            }
            dbg!(&possible_numbers);
        }
    possible_numbers[0].to_string()
}
