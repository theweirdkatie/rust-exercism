pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut result: Vec<Vec<u32>> = (0..size).map(|_| (0..size).map(|_| 0).collect()).collect();
    let (mut row, mut col): (i32, i32) = (0, 0);
    let (mut d_row, mut d_col): (i32, i32) = (0, 1);

    for count in 1..=(size*size) {
        result[row as usize][col as usize] = count;
        row += d_row;
        col += d_col;

        // if col # is =size or more OR the col direction is positive and the next spot has been filled
        // Col movement freezes (=0), row direction becomes positive, col backs off and row increases
        // east to south
        if col >= size as i32 || (d_col == 1 && result[row as usize][col as usize] != 0) {
            (d_row, d_col) = (1, 0);
            col -= 1; row += 1;
        
        // if row # is =size or more OR the row direction is positive and the next spot has been filled
        // Row movement freezes (=0), col direction becomes negative, row backs off and col backs off
        // south to west
        } else if row >= size as i32 || (d_row == 1 && result[row as usize][col as usize] != 0) {
            (d_row, d_col) = (0, -1);
            row -= 1; col -= 1;
        
        // if col # is < 0 OR the col direction is negative and the next spot has been filled
        // Col movement freezes (=0), row direction becomes negative, col corrects and row decreases
        // west to north
        } else if col < 0 || (d_col == -1 && result[row as usize][col as usize] != 0) {
            (d_row, d_col) = (-1, 0);
            col += 1; row -= 1;
        
        // if row # is < 0 OR the row direction is negative and the next spot has been filled
        // Row movement freezes (=0), col direction becomes positive, row corrects and col increases
        // north to east
        } else if row < 0 || (d_row == -1 && result[row as usize][col as usize] != 0) {
            (d_row, d_col) = (0, 1);
            row += 1; col += 1;
        }
    }
    result
}
