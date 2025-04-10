use std::thread;

fn main() {
    let x = 1;

    let handle = thread::spawn(move || {
        println!("hello world: {:?}", x);
    });

    handle.join().unwrap();

    let handle = thread::spawn(move || {
        println!("hello world: {:?}", x);
    });

    handle.join().unwrap();
}
