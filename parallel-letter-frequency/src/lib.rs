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
