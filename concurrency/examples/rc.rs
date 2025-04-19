use std::{cell::RefCell, collections::HashMap, rc::Rc};

fn rc_refcell_example() {
    let shared_map = Rc::new(RefCell::new(HashMap::new()));

    {
        let mut map = shared_map.borrow_mut();
        map.insert("africa", 92388);
        map.insert("kyoto", 11837);
        map.insert("piccadilly", 11826);
        map.insert("marbles", 38);
    }

    let total = shared_map.borrow().values().sum::<i32>();
    println!("total: {}", total);
}

fn main() {
    let data = Rc::new(100);

    let r1 = data.clone();
    let r2 = data.clone();

    println!("{}", r1);
    println!("{}", r2);

    rc_refcell_example();
}
