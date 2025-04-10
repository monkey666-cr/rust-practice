use std::{thread, time::Duration};

fn main() {
    println!("Hello, world!");

    let h = thread::spawn(|| {
        thread::sleep(Duration::from_secs(1));
        let result = std::panic::catch_unwind(|| {
            panic!("boom");
        });
        println!("panic caught: {}", result.is_err());
    });

    let r = h.join();
    match r {
        Ok(r) => println!("thread returned {:?}", r),
        Err(e) => println!("thread panicked with {:?}", e),
    }
}
