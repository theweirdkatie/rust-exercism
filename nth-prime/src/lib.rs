pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2, 3, 5];
    while primes.len() <= n as usize {
        let mut next_prime = primes.last().unwrap() + 2;
        while primes.iter().any(|x| next_prime % x == 0) {
            next_prime += 2;
        }
        primes.push(next_prime);
    }
    primes[n as usize]
}
