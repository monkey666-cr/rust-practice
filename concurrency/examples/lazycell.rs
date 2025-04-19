use std::cell::LazyCell;

fn main() {
    let lazy = LazyCell::new(|| {
        println!("initializing...");
        40
    });

    println!("ready");
    println!("{}", *lazy);
    println!("{}", *lazy);
}
