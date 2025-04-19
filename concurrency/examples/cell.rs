use std::cell::Cell;

fn main() {
    let x = Cell::new(452);
    let y = &x;

    x.set(10);

    println!("y: {}", y.get());
}
