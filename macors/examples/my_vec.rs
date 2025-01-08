use macors::my_vec;

fn main() {
    let v = my_vec!(1, 2, 3);
    println!("{:?}", v);

    let v2 = vec![5; 10];
    println!("{:?}", v2);
}
