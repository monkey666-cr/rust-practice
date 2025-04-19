use std::{
    sync::{Arc, RwLock},
    thread,
};

fn main() {
    let counter = Arc::new(RwLock::new(100));

    let mut read_handlers = vec![];

    for _ in 0..3 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let num = counter.read().unwrap();
            println!("Reader {:?}: {}", thread::current().id(), *num);
        });
        read_handlers.push(handle);
    }

    let write_handle = thread::spawn(move || {
        let mut num = counter.write().unwrap();
        *num += 1;
        println!(
            "Writer {:?}: Incremented counter to {}",
            thread::current().id(),
            *num
        );
    });

    for handle in read_handlers {
        handle.join().unwrap();
    }
    write_handle.join().unwrap();
}
