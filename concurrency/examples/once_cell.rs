use std::sync::OnceLock;

static CELL: OnceLock<String> = OnceLock::new();

fn main() {
    std::thread::spawn(|| {
        let value = CELL.get_or_init(|| "hello, world".to_string());
        assert_eq!(value, "hello, world");
    })
    .join()
    .unwrap();

    let value = CELL.get();
    assert!(value.is_some());
    assert_eq!(value.unwrap().as_str(), "hello, world");
}
