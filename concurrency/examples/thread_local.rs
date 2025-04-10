use std::{cell::RefCell, thread};

fn main() {
    thread_local!(static COUNTER: RefCell<u32> = RefCell::new(1));

    COUNTER.with(|counter| {
        *counter.borrow_mut() += 1;
    });

    let handle1 = thread::spawn(move || {
        COUNTER.with(|counter| {
            *counter.borrow_mut() = 3;
        });
    });
    let handle2 = thread::spawn(move || {
        COUNTER.with(|counter| {
            *counter.borrow_mut() = 4;
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    COUNTER.with(|counter| {
        println!("c={:?}", counter);
    });
}
