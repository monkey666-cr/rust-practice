use std::thread;
use std::time::Duration;
use thread_control::*;

fn main() {
    let (flag, control) = make_pair();
    let handle = thread::spawn(move || {
        while flag.alive() {
            println!("hello world");
        }
    });
    thread::sleep(Duration::from_secs(1));
    assert_eq!(control.is_done(), false);
    control.stop();
    handle.join().unwrap();
    assert_eq!(control.is_interrupted(), false);
    assert_eq!(control.is_done(), true);
}
