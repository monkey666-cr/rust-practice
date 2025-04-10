use std::thread;
use std::time::Duration;

fn main() {
    let parked_thread = thread::spawn(|| {
        println!("Parking thread");
        thread::park();
        println!("Thread unparked");
    });

    thread::sleep(Duration::from_millis(10));

    println!("Unparking thread");
    parked_thread.thread().unpark();

    parked_thread.join().unwrap();
}
