use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

fn main() {
    let atomic_bool = Arc::new(AtomicBool::new(false));

    let producer_thread = {
        let atomic_bool = atomic_bool.clone();
        thread::spawn(move || {
            // 这里可能有指令重排
            atomic_bool.store(true, Ordering::Release);
        })
    };

    let consumer_thread = {
        let atomic_bool = atomic_bool.clone();
        thread::spawn(move || {
            // 等待直到读取到布尔值为true
            while !atomic_bool.load(Ordering::Acquire) {
                // 这里可能进行自旋，知道读取到Acquire顺序的布尔值。
                // 注意：在实际应用中，可能使用高级的同步源于而不是使用自旋。
            }
            println!("Received value");
        })
    };

    producer_thread.join().unwrap();
    consumer_thread.join().unwrap();
}
