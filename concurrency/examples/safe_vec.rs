use std::sync::{Arc, Mutex};

fn main() {
    let shared_vec = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];
    for i in 0..5 {
        let shared_vec = Arc::clone(&shared_vec);
        let handle = std::thread::spawn(move || {
            let mut vec = shared_vec.lock().unwrap();
            vec.push(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_vec = shared_vec.lock().unwrap();
    println!("Final Vec: {final_vec:?}");
}
