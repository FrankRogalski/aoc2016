use std::fs;
use std::error::Error;

#[derive(Default)]
struct Pos {
    x: i32,
    y: i32,
    dir: u256,
}

fn part1() -> Box<dyn Error> {
    let res = fs::read_to_string("resources/day01.txt")?;
    let mut pos = Pos::default();
    res.split(" ").into_iter()
        .map(
}

fn main() {
}
