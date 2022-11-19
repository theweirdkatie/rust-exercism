#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
   if _first_list == _second_list {
    Comparison::Equal

    // Finding sub or superlist doesn't work if slice len == 0, catch before running
   } else if _first_list.len() == 0 {
    Comparison::Sublist
   } else if _second_list.len() == 0 {
    Comparison::Superlist
    
    // Windows(len) returns an iterator over all contiguous windows of len, i.e. iterates over vectors of size
    // len in slice
    // With any() checks any of the iterators against a predicated and returns true or false
    // any(occurance, predicate)
   } else if _second_list.windows(_first_list.len()).any(|x| x == _first_list) {
    Comparison::Sublist
   } else if _first_list.windows(_second_list.len()).any(|x| x== _second_list) {
    Comparison::Superlist
   } else {
    Comparison::Unequal
   }
}

