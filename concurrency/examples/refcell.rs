use std::{cell::RefCell, ops::Deref};

fn main() {
    let x = RefCell::new(42);

    {
        let y = x.borrow();
        println!("y: {}", y);
    }

    {
        let mut z = x.borrow_mut();
        *z = 10;
    }

    println!("x: {:?}", x.borrow().deref());
}
