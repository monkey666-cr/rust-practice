use std::thread;

fn create_thread() {
    let current_thread = thread::current();
    println!(
        "current thread id: {:?}, {:?}",
        current_thread.id(),
        current_thread.name()
    );

    let builder = thread::Builder::new()
        .name("foo".into())
        .stack_size(32 * 1024);
    let handler = builder
        .spawn(|| {
            let current_thread = thread::current();
            println!(
                "current thread id: {:?}, {:?}",
                current_thread.id(),
                current_thread.name()
            );
        })
        .unwrap();

    handler.join().unwrap();
}

fn main() {
    create_thread();
}
