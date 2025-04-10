use std::thread;

pub fn start_one_thread_by_builder() {
    let builder = thread::Builder::new()
        .name("foo".into())
        .stack_size(32 * 1024);

    let handler = builder
        .spawn(|| {
            println!("Hello from a thread");
        })
        .unwrap();

    handler.join().unwrap();
}

fn main() {
    start_one_thread_by_builder();
}
