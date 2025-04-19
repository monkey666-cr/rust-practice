use std::{
    sync::{Arc, Barrier},
    time::Duration,
};

fn main() {
    let barrier = Arc::new(Barrier::new(3));

    let mut handles = vec![];

    for id in 0..3 {
        let barrier = Arc::clone(&barrier);
        let handle = std::thread::spawn(move || {
            println!("Thread {} working", id);
            std::thread::sleep(Duration::from_secs(id as u64));

            barrier.wait();

            println!("Thread {} resumed", id);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
