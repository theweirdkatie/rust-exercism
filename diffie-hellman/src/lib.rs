use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modulus(g.into(), a.into(), p.into())
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modulus(b_pub.into(),a.into(),p.into())
}

fn modulus(b: u128, e: u128, m: u128) -> u64 {
    let mut result = 1;
    let mut b = b % m;
    let mut e = e;
    if m == 1 {
        return 0;
    }

    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % m;
        }
        e = e >> 1;
        b = b.pow(2) % m;
    }
    result as u64
}
