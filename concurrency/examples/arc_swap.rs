use std::sync::Arc;

use arc_swap::ArcSwap;

fn main() {
    let data = ArcSwap::new(Arc::new(1));

    println!("Initial Value: {}", data.load());

    data.store(Arc::new(2));

    println!("New Value: {}", data.load());
}
