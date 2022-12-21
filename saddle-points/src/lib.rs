pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for (i_row, row) in input.iter().enumerate() {
        for (i_col, value) in row.iter().enumerate() {
            if row.iter().all(|x| x <= value) && (0..input.len()).all(|x| input[x][i_col] >= *value) {
                result.push((i_row, i_col));
            }
        }
    }
    result
}
