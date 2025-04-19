use std::cell::OnceCell;

fn main() {
    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    let value = cell.get_or_init(|| "Hello".to_string());
    assert_eq!(value, "Hello");
    assert!(cell.get().is_some());
}
