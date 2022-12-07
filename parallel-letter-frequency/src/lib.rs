/*
not great
test bench_large_parallel   ... bench:  48,938,200 ns/iter (+/- 68,820,217)
test bench_large_sequential ... bench:     640,000 ns/iter (+/- 23,729)
test bench_small_parallel   ... bench:   1,627,537 ns/iter (+/- 104,106)
test bench_small_sequential ... bench:      22,393 ns/iter (+/- 819)
test bench_tiny_parallel    ... bench:     145,720 ns/iter (+/- 18,039)
test bench_tiny_sequential  ... bench:         128 ns/iter (+/- 15)
 */

use std::{
    collections::HashMap,
    thread,
    sync::{Arc, Mutex},
    ops::Deref,
};

pub fn frequency(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    let mut workers = vec![];
    let map = Arc::new(Mutex::new(HashMap::new()));
    let cloned_input: Vec<String> = input.iter().map(|&x| x.clone().to_owned().to_lowercase()).collect();

    for sentence in cloned_input {
        let clone_arc_map = Arc::clone(&map);
        let worker = thread::spawn(move || {
            let mut map = clone_arc_map.lock().unwrap();
            for c in sentence.chars(){
                if c.is_alphabetic() {
                    let count = map.entry(c).or_insert(0);
                    *count += 1;
                }
            }
        });
        workers.push(worker);
    }

    for worker in workers {
        worker.join().unwrap();
    }

    let mut result: HashMap<char, usize> = HashMap::new();
    result.extend(map.as_ref().lock().expect("dxfgh").deref());
    result
}