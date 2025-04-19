use std::borrow::Cow;

fn main() {
    let origin = String::from("hello");
    let mut cow = Cow::from(&origin);
    assert_eq!(cow, "hello");

    let s = &cow;
    assert_eq!(s, "hello");
    assert_eq!(s.len(), cow.len());

    let s: &mut str = cow.to_mut();
    s.make_ascii_uppercase();
    assert_eq!(s, "HELLO");
    assert_eq!(origin, "hello");
}
