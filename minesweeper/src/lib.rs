pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield                                           // take original array
        .iter()                                      // create iterator
        .enumerate()           // enumerate
        .map(|index, row| {                                     // map the index and row data
            row.char_indices()                                  // where row data is indexed by character
                .map(|col, val| match val {                     // and mapped to a column index and a single character value
                    '*' => '*',                                 // where if the value is a bomb, it stays a bomb
                    _ => count_bombs(minefield, index, col),    // but if it's anything else, count the bombs around that square
                })
                .collect::<String>()                            // collect these values as a Vec<String> for the row
        })
        .collect()                                              // collect all these values/rows into vector and return value
}

pub fn count_bombs(field: &[&str], row_num: usize, col_num:usize ) -> char {
    let mut bombs = 0;
    let range_start = if row_num > 0 {row_num-1} else {0};
    let range_end = if row_num < (field.len() -1) {row_num + 2} else {field.len()};

    bombs = field[(range_start..range_end)].iter().map(|&x| )
}
