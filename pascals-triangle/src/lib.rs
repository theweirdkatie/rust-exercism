pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        println!("Row count: {}", self.row_count);
        (0..self.row_count).map(|row| PascalsTriangle::generate_row(row)).collect()
    }

    pub fn generate_row(row_num: u32) -> Vec<u32> {
        let mut result = vec![1];
        for col in 1..=row_num {
            println!("Here!");
            if let Some(last) = result.last() {
                result.push((last*(row_num+1 - col))/col);
            }
        }
        result
    }
}
