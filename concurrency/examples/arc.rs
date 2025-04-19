use std::{sync::Arc, thread};

fn main() {
    let data = Arc::new(100);

    let thread1 = {
        let data = Arc::clone(&data);

        thread::spawn(move || {
            println!("thread1: {}", data);
        })
    };

    let thread2 = {
        let data = Arc::clone(&data);

        thread::spawn(move || {
            println!("thread2: {}", data);
        })
    };

    thread1.join().unwrap();
    thread2.join().unwrap();
}
