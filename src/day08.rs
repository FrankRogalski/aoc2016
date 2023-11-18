use std::fs;
use std::error::Error;

fn place_grid(grid: &mut [[bool; 50]; 6], dim: &str) {
    let Some((width, height)) = dim.split_once('x') else {
        unimplemented!();
    };
    let width: usize = width.parse().unwrap();
    let height: usize = height.parse().unwrap();
    for x in 0..width {
        for y in 0..height {
            grid[y][x] = true;
        }
    }
}

fn shift_row(grid: &mut [[bool; 50]; 6], args: Vec<&str>) {
    let shift: usize = args.last().unwrap().parse().unwrap();
    let row: usize = args.first().unwrap().split('=').last().unwrap().parse().unwrap();
    grid[row].rotate_right(shift);
}

fn shift_column(grid: &mut [[bool; 50]; 6], args: Vec<&str>) {
    let shift: usize = args.last().unwrap().parse().unwrap();
    let col: usize = args.first().unwrap().split('=').last().unwrap().parse().unwrap();
    for _ in 0..shift {
        let mut last = grid.last().unwrap()[col];
        for y in 0..6 {
            let temp = grid[y][col];
            grid[y][col] = last;
            last = temp;
        }
    }
}

fn compute_grid(input: &str) -> [[bool; 50]; 6] {
    let mut grid = [[false; 50]; 6];
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let first = parts.next();
        let second = parts.next();
        match (first.unwrap(), second.unwrap()) {
            ("rect", dim) => place_grid(&mut grid, dim),
            ("rotate", "row") => shift_row(&mut grid, parts.collect()),
            ("rotate", "column") => shift_column(&mut grid, parts.collect()),
            _ => {}
        }
    }
    grid
}

fn part1() -> Result<(), Box<dyn Error + 'static>> {
    let res = fs::read_to_string("resources/day08.txt")?;
    let grid = compute_grid(&res);
    let mut count = 0;
    for y in 0..6 {
        for x in 0..50 {
            if grid[y][x] {
                count += 1;
            }
        }
    }
    println!("part1 has {count} leds on");
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error + 'static>> {
    let res = fs::read_to_string("resources/day08.txt")?;
    let grid = compute_grid(&res);
    println!("part 2 writes this:");
    for y in 0..6 {
        for x in 0..50 {
            if grid[y][x] {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + 'static>>{
    part1()?;
    part2()?;
    Ok(())
}
