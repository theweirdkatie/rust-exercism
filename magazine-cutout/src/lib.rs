// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    
    let mut magazine_map = magazine.iter().fold(HashMap::new(),|mut hm, word| {*hm.entry(word).or_insert(0) += 1; hm});
    
    for word in note {
        let entry = magazine_map.entry(word).or_insert(0);
        if *entry == 0 {
            return false;
        }
        *entry -= 1;
    }
    true
}
