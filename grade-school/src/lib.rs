// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
use std::collections::HashMap;

#[allow(clippy::new_without_default)]
#[derive(Default, Debug)]
pub struct School {
    grades: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            ..Default::default()
        }
    }

    pub fn add(&mut self, _grade: u32, student: &str) {
        // entry(key) returns enum Entry for key, Occupied or Vacant
        // If vacant, or_default() adds key with default value (empty vector)
        // and returns a reference to the value, the empty vector, which can be
        // used to push the new name
        self.grades.entry(_grade)
                    .or_default()
                    .push(student.to_owned());
    }

    pub fn grades(&self) -> Vec<u32> {
        // Copy and collect keys into vector, then sort
        let mut result: Vec<u32> = self.grades.keys().copied().collect();
        result.sort();
        result
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, _grade: u32) -> Vec<String> {
        // Get vector value from key _grade returns Option<value>
        // unwrap or provide empty vector
        // reference convert to vector which can be sorted
        let mut result: Vec<String> = self.grades.get(&_grade).unwrap_or(&Vec::new()).to_vec();
        result.sort();
        result
    }
}
