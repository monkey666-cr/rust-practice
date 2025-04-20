use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let shared_map = Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];
    for i in 0..5 {
        let shared_map = shared_map.clone();
        let handle = thread::spawn(move || {
            let mut map = shared_map.lock().unwrap();
            map.insert(i, i * i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_map = shared_map.lock().unwrap();
    println!("Final hashmap: {final_map:?}");
}
