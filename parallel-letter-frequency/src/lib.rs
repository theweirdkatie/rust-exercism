/*
better
test bench_large_parallel   ... bench:     614,383 ns/iter (+/- 70,580)
test bench_large_sequential ... bench:     678,255 ns/iter (+/- 102,993)
test bench_small_parallel   ... bench:     293,888 ns/iter (+/- 32,422)
test bench_small_sequential ... bench:      23,781 ns/iter (+/- 443)
test bench_tiny_parallel    ... bench:     160,651 ns/iter (+/- 19,066)
test bench_tiny_sequential  ... bench:         133 ns/iter (+/- 2)
 */

use std::{
    collections::HashMap,
    thread,
    sync::{Arc, Mutex},
};

pub fn frequency(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    let mut workers = vec![];
    let map = Arc::new(Mutex::new(HashMap::new()));
    let chunks = input.chunks((input.len() / _worker_count).max(1));

    for chunk in chunks {
        let clone_arc_map = map.clone();
        let string = chunk.join("");
        let worker = thread::spawn(move || {
            let mut map = clone_arc_map.lock().unwrap();
            for c in string.chars() {
                if c.is_alphabetic() {
                    let count = map.entry(c.to_ascii_lowercase()).or_insert(0);
                    *count += 1;
                }
            }
        });
        workers.push(worker);
    }

    for worker in workers {
        worker.join().unwrap();
    }

    Arc::try_unwrap(map).unwrap().into_inner().unwrap()
}