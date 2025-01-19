#![allow(unused)]

use macors::EnumFromDarling;

#[derive(Debug, EnumFromDarling)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right { a: u32 },
}

#[derive(Debug)]
struct DirectionUp {
    speed: i32,
}

impl DirectionUp {
    fn new(speed: i32) -> Self {
        Self { speed }
    }
}

fn main() {
    let up: Direction = DirectionUp::new(42).into();
    let left: Direction = 10.into();
    println!("{:?}, {:?}", up, left);
}
