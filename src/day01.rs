use std::fs;
use std::error::Error;
use std::collections::hash_set::HashSet;

#[derive(Default, Debug)]
enum Dir {
    #[default]
    NORTH,
    SOUTH,
    EAST,
    WEST
}
#[derive(Default, Debug)]
struct Pos {
    x: i32,
    y: i32,
    dir: Dir,
}

fn left(dir: &Dir) -> Dir {
    match dir {
        Dir::NORTH => Dir::WEST,
        Dir::WEST => Dir::SOUTH,
        Dir::SOUTH => Dir::EAST,
        Dir::EAST => Dir::NORTH,
    }
}

fn right(dir: &Dir) -> Dir {
    match dir {
        Dir::NORTH => Dir::EAST,
        Dir::EAST => Dir::SOUTH,
        Dir::SOUTH => Dir::WEST,
        Dir::WEST => Dir::NORTH,
    }
}

fn turn(dir_str: Option<char>, dir: &Dir) -> Dir {
    match dir_str {
        Some('R') => right(&dir),
        Some('L') => left(&dir),
        _ => panic!("wtf")
    }
}

fn part1() -> Result<(), Box<dyn Error + 'static>> {
    let res = fs::read_to_string("resources/day01.txt")?;
    let mut pos = Pos::default();
    for dir in res.split(",") {
        let mut chars = dir.chars();
        pos.dir = turn(chars.next(), &pos.dir);
        let steps :i32 = chars.collect::<String>()
            .replace("\n", "")
            .parse()
            .expect("this to be a number");
        match pos.dir {
            Dir::NORTH => pos.y -= steps,
            Dir::EAST => pos.x += steps,
            Dir::SOUTH => pos.y += steps,
            Dir::WEST => pos.x -= steps,
        }
    }
    println!("part1");
    let dist = pos.x.abs() + pos.y.abs();
    println!("{pos:?}, distance from 0,0 = {dist}");

    Ok(())
}

fn inner_loop(positions: &mut HashSet<(i32, i32)>, i: i32, other: i32, x_axis: bool) -> bool {
    if x_axis {
        if !positions.insert((i, other)) {
            let dist = i.abs() + other.abs();
            println!("paths crossed at {i}, {other} distance is {dist}");
            return true;
        }
    } else {
        if !positions.insert((other, i)) {
            let dist = i.abs() + other.abs();
            println!("paths crossed at {other}, {i} distance is {dist}");
            return true;
        }
    }
    false
}

fn duplicate_in_positions(positions: &mut HashSet<(i32, i32)>, from: i32, to: i32, other: i32, x_axis: bool) -> bool {
    if from > to {
        let mut i = from;
        while i > to {
            if inner_loop(positions, i, other, x_axis) {
                return true;
            }
            i -= 1;
        }
    } else {
        for i in from..to {
            if inner_loop(positions, i, other, x_axis) {
                return true;
            }
        }
    }
    false
}

fn part2() -> Result<(), Box<dyn Error + 'static>> {
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    let res = fs::read_to_string("resources/day01.txt")?;
    let mut pos = Pos::default();
    println!("part2");
    for dir in res.split(",") {
        let mut chars = dir.chars();
        pos.dir = turn(chars.next(), &pos.dir);
        let steps :i32 = chars.collect::<String>()
            .replace("\n", "")
            .parse()
            .expect("this to be a number");
        match pos.dir {
            Dir::NORTH => {
                if duplicate_in_positions(&mut positions, pos.y, pos.y - steps, pos.x, false) {
                    return Ok(());
                }
                pos.y -= steps;
            },
            Dir::EAST => {
                if duplicate_in_positions(&mut positions, pos.x, pos.x + steps, pos.y, true) {
                    return Ok(());
                }
                pos.x += steps;
            },
            Dir::SOUTH => {
                if duplicate_in_positions(&mut positions, pos.y, pos.y + steps, pos.x, false) {
                    return Ok(());
                }
                pos.y += steps;
            },
            Dir::WEST => {
                if duplicate_in_positions(&mut positions, pos.x, pos.x - steps, pos.y, true) {
                    return Ok(());
                }
                pos.x -= steps;
            },
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + 'static>>{
    part1()?;
    part2()?;
    Ok(())
}
