pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // Minefield is an array of string slices
    // Iterate through each element of the array ("row")
    // Iterate through each character of the slice ("column")
    // If a mine, map character for a mine back - if not a mine, calculate the number of adjacent mines
    // Collect characters into a string per row, store in vector of rows
    minefield
        .iter()
        .enumerate()
        .map(|(row_num, row)| {
            row
                .chars()
                .enumerate()
                .map(|(col_num, cell)| {
                    match cell {
                        '*' => '*',
                        _ => count_mines(minefield, row_num, col_num),
                    }
                })
        .collect::<String>()
        })
        .collect()
}

pub fn count_mines(field: &[&str], row_num: usize, col_num: usize) -> char {
    let mut mines = 0;
    // Define checking range for referenced row
    // Can check row before and after
    let row_range = std::ops::Range{
        start: if row_num==0 {0} else {row_num-1},
        end: if row_num==field.len()-1 {row_num+1} else {row_num+2}, // end is exclusive
    };

    // For each row defined in the range of adjacent rows (including own row)
    for row in field.get(row_range).unwrap_or_default() {
        // Define checking range for referenced column
        // Can check columns before and after
        let col_range = std::ops::Range{
            start: if col_num==0 {0} else {col_num-1},
            end: if col_num==row.len()-1 {col_num+1} else {col_num+2} // end is exclusive
        };

        // For each cell in the row within the column range (including own column)
        for cell in row.get(col_range).unwrap_or_default().chars() {
            // If the cell has a mine, increase count
            if cell=='*' {mines += 1} else {}; 
        }
    }

    // Return character version of number unless it is zero - zeros are blank spaces ' '
    if mines==0 {' '} else {char::from_digit(mines, 10).unwrap_or(' ')}
}
