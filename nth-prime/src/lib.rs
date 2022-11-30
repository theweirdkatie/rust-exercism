pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2, 3, 5];
    let (mut a, mut k, mut p): (u32, u32, u32) = (5, 1, 0);

    while primes.len()-1 < n as usize {
        p = a + 2*k;
        if !(is_prime(p, &primes)){
            k += 1;
        } else {
            primes.push(p as u32);
            a = p;
            k = 1;
        }
    }
    primes[n as usize]
}

pub fn is_prime(p: u32, primes: &[u32]) -> bool {
    let s = ((p-1) as f64/2.0).round() as u32;
    for i in primes.iter(){
        if i > &s {
            return true;
        } else if p % i == 0 {
            return false;
        } else {};
    }
    true
}