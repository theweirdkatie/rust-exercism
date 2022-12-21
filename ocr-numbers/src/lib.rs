// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use std::vec;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let vec_input = input.split('\n').collect::<Vec<&str>>();
    check_valid(vec_input.clone())?;

    let mut result = String::new();
    for col in 0..vec_input[0].len()/3 {
        for mul_line in 0..vec_input.len()/4 {
            let mut temp: Vec<&str> = vec![];
            for row in mul_line*4..(mul_line*4)+4 {
                println!("{row}/{}", (mul_line*4)+4);
                temp.push(&vec_input[row][col*3..(col*3)+3]);
            }
            result = result + &get_num(temp);
        }
        println!("{result}");
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
    input.iter().any(|&x| !["| |", "   ", "  |", " _ ", "|_ ", " _|", "|_|"].contains(&x))
}

pub fn get_num(number: Vec<&str>) -> String {
    let mut possible_numbers = (0..10).collect::<Vec<usize>>();
    for (i, row) in number.iter().enumerate() {
        println!("match {row} at {i}");
        match i {
            0 => {
                match *row {
                    "   " => possible_numbers.retain(|x| [1,4].contains(x)),
                    " _ " => possible_numbers.retain(|x| ![1,4].contains(x)),
                    _ => return "?".to_string(),
                }
            }
            1 => {
                match *row {
                    "  |" => possible_numbers.retain(|x| [1,7].contains(x)),
                    " _|" => possible_numbers.retain(|x| [2,3].contains(x)),
                    "|_|" => possible_numbers.retain(|x| [4,8,9].contains(x)),
                    "|_ " => possible_numbers.retain(|x| [5,6].contains(x)),
                    "| |" => {
                        if !possible_numbers.contains(&0) || number[2] != "|_|" {
                            return "?".to_string();
                        } else {
                            return "0".to_string();
                        }
                    }
                    _ => return "?".to_string(),
                }
            }
            2 => {
                match *row {
                    "  |" => possible_numbers.retain(|x| [1,4,7].contains(x)),
                    " _|" => possible_numbers.retain(|x| [3,5,9].contains(x)),
                    "|_|" => possible_numbers.retain(|x| [8,6,0].contains(x)),
                    "|_ " => possible_numbers.retain(|&x| x==2),
                    _ => return "?".to_string(),
                }
            }
            _ => {
                match *row {
                    "   " => {},
                    _ => return "?".to_string(),
                }
            }
        }
    }
    dbg!(&possible_numbers);
    possible_numbers[0].to_string()
}
