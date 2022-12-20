pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return vec![];
    }
    let mut is_prime: Vec<bool> = (2..=upper_bound).map(|_| true).collect();
    for n in 2..=(upper_bound as f64).sqrt() as usize {
        if is_prime[n - 2] {
            let mut i = 0;
            let mut num: usize = 2 * n;
            while num <= upper_bound as usize {
                is_prime[num - 2] = false;
                i += 1;
                num = 2 * n + i * n;
            }
        }
    }
    is_prime
        .iter()
        .enumerate()
        .map(|(idx, &val)| if val { (idx + 2) as u64 } else { 0_u64 })
        .filter(|&x| x > 0)
        .collect::<Vec<u64>>()
}
