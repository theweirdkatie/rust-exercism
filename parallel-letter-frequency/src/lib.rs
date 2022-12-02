use std::{
    collections::HashMap,
    thread,
    sync::{Arc, Mutex},
};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut workers = vec![];
    let map: HashMap<char, usize> = HashMap::new();
    let arc_map = Arc::new(Mutex::new(map));

    for sentence in input {
        let clone_arc_map = Arc::clone(&arc_map);
        let worker = thread::spawn(move || {
            let mut map = clone_arc_map.lock().unwrap();
            for c in sentence.chars(){
                let count = map.entry(c).or_insert(0);
                *count += 1;
            }
        });
        workers.push(worker);
    }

    for worker in workers {
        worker.join().unwrap();
    }

    map
}
