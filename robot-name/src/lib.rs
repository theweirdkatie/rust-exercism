use lazy_static::lazy_static;
use rand::random;
use std::{collections::HashSet, sync::Mutex};

pub struct Robot {
    name: String,
}

lazy_static! {
    static ref All_Names: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

impl Robot {
    pub fn new() -> Self {
        let first_char: char =
            ('A'..'Z').collect::<Vec<char>>()[random::<usize>() % (24 + 1 - 0) + 0];
        let second_char: char =
            ('A'..'Z').collect::<Vec<char>>()[random::<usize>() % (24 + 1 - 0) + 0];
        let num: u32 = random::<u32>() % (999 + 1 - 100) + 100;
        let name = format!("{}{}{}", first_char, second_char, num);
        if All_Names.lock().unwrap().contains(&name) {
            Robot::new()
        } else {
            All_Names.lock().unwrap().insert(name.clone());
            Self { name }
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        All_Names.lock().unwrap().remove(&self.name);
        *self = Robot::new();
    }
}
