use std::thread;

fn start_one_thread() {
    let handle: thread::JoinHandle<()> = thread::spawn(|| {
        println!("Hello from a thread");
    });

    handle.join().unwrap();
}

fn start_n_thread() {
    const N: usize = 10;
    let handles = (0..N)
        .map(|i| {
            thread::spawn(move || {
                println!("Hello from a thread {}", i);
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    start_one_thread();
    start_n_thread();
}
