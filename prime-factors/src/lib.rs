pub fn factors(mut n: u64) -> Vec<u64> {
    let mut facts = vec![];
    let mut p: u64 = 2;

    if n<=1 {
        facts
    } else {
        loop {
            if n < p.pow(2) {
                facts.push(n);
                break;
            } else if n % p == 0 {
                facts.push(p);
                n = n / p;
                println!("{}", p);
            } else {
                p += 1;
            }
        }
        facts
    }
}
