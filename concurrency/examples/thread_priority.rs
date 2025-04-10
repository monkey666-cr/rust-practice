use std::{thread, time::Duration};

use thread_priority::{set_current_thread_priority, ThreadPriority};

fn main() {
    let handle1 = thread::spawn(|| {
        assert!(
            set_current_thread_priority(ThreadPriority::Crossplatform(0.try_into().unwrap()))
                .is_ok()
        );
        thread::sleep(Duration::from_secs(1));
        println!("hello world");
    });

    let handle2 = thread::spawn(|| {
        assert!(
            set_current_thread_priority(ThreadPriority::Crossplatform(1.try_into().unwrap()))
                .is_ok()
        );
        thread::sleep(Duration::from_secs(1));
        println!("hello rust");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
