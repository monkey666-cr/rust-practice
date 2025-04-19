use std::{
    sync::{Arc, Barrier},
    time::Duration,
};

use rand::Rng;

fn main() {
    let barrier = Arc::new(Barrier::new(10));
    let mut handles = vec![];

    for _ in 0..10 {
        let barrier = Arc::clone(&barrier);
        handles.push(std::thread::spawn(move || {
            println!("before wait1");
            let dur = rand::thread_rng().gen_range(100..1000);
            std::thread::sleep(Duration::from_millis(dur));

            barrier.wait();

            println!("after wait1");
            std::thread::sleep(Duration::from_secs(1));

            barrier.wait();
            println!("after wait2");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
