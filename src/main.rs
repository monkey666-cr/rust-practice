mod algorithms;

use algorithms::linked_list::List;

fn main() {
    let mut l = List::new();

    l.push(1);
    l.push(1);
    l.push(1);
    l.push(1);

    for v in l.iter_mut() {
        *v = 10;
    }

    for v in l.iter() {
        println!("{}", v);
    }
}
