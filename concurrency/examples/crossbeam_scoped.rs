fn main() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    crossbeam::scope(|s| {
        s.spawn(|_| {
            println!("hello from thread");
            dbg!(&a);
        });
        s.spawn(|_| {
            println!("hello from thread");
            x += a[0] + a[2];
        });
        println!("hello from main");
    })
    .unwrap();

    a.push(4);
    assert_eq!(x, a.len());
}
