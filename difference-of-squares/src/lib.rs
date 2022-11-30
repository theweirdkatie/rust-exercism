pub fn square_of_sum(n: u32) -> u32 {
    // Create an inclusive range from 1 to n
    // Ranges are iterators, so sum each iteration
    // Square the resulting number
    (1..=n).sum::<u32>().pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    // Create an inclusive range from 1 to n
    // Ranges are iterators, so square each iteration
    // Sum all the squares
    (1..=n).map(|x| x.pow(2)).sum::<u32>()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
