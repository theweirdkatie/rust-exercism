use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let arguments: Vec<&str> = input
                                .split(|x: char| !x.is_alphabetic())
                                .filter(|y| !y.is_empty())
                                .collect();
    dbg!(&arguments);
    
    
    
    Some(HashMap::new())
}
