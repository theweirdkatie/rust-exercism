use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut set = HashSet::new();
    for i in 1..=(sum-3) {
        for j in (i+1)..=(sum/2) {
            let k = sum - i - j;
            if i*i + j*j == k*k {
                set.insert([i,j,k]);
            }
        }
    }
    set
}
