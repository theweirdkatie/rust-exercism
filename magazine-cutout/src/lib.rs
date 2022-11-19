// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_map = HashMap::new();

    // add all words in magazine to a hash map
    for word in magazine {
        let count = magazine_map.entry(word).or_insert(0);
        *count += 1;
    }

    // iterate over all words in note
    for word in note {
        let entry = magazine_map.entry(word).or_insert(0);

        if *entry == 0 {
            return false;
        }

        *entry -= 1;
    }

    true
}
