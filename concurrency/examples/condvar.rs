use std::{
    sync::{Arc, Condvar, Mutex},
    time::Duration,
};

fn main() {
    let mutex = Arc::new(Mutex::new(false));
    let condvar = Arc::new(Condvar::new());

    let mut handles = vec![];

    for id in 0..3 {
        let mutex = Arc::clone(&mutex);
        let condvar = Arc::clone(&condvar);

        let handle = std::thread::spawn(move || {
            let mut guard: std::sync::MutexGuard<'_, bool> = mutex.lock().unwrap();

            while !*guard {
                guard = condvar.wait(guard).unwrap();
            }

            println!("Thread {} woke up", id);
        });

        handles.push(handle);
    }

    std::thread::sleep(Duration::from_secs(2));

    {
        let mut guard = mutex.lock().unwrap();
        *guard = true;
        condvar.notify_all();
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
