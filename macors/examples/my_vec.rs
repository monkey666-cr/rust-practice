fn main() {
    let v: Vec<i32> = my_vec!(1, 2, 3);
    println!("{:?}", v);

    let v2 = vec![5; 10];
    println!("{:?}", v2);
}

#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };
    ($($x:expr),+ $(,)?) => {
        {
            <[_]>::into_vec(Box::new([$($x),*]))
        }
    };
}
