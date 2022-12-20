pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut numbers: Vec<u64> = (2..=upper_bound).collect();
    for num in 2..=(upper_bound as f64).sqrt() as u64 {
        numbers.retain(|&x| x % num != 0 || x == num);
    }
    numbers
}
