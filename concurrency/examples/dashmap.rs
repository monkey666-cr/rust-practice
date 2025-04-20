use std::sync::Arc;

use dashmap::DashMap;

fn main() {
    let map = Arc::new(DashMap::new());
    let mut handles = vec![];

    for i in 0..10 {
        let map = map.clone();
        handles.push(std::thread::spawn(move || {
            map.insert(i, i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("DashMap: {map:?}");
}
