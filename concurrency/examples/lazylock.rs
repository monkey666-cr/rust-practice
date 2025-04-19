use std::{collections::HashMap, sync::LazyLock};

static HASHMAP: LazyLock<HashMap<i32, String>> = LazyLock::new(|| {
    println!("initializing...");
    let mut m = HashMap::new();
    m.insert(18, "hello 18".to_string());
    m.insert(10, "hello 10".to_string());
    m
});

fn main() {
    println!("ready");
    std::thread::spawn(|| {
        println!("hashmap: {:?}", HASHMAP.get(&18));
    })
    .join()
    .unwrap();
    print!("{:?}", HASHMAP.get(&10));
}
