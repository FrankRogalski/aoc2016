use std::fs;
use std::error::Error;

const P1_PANEL: [[u8; 3]; 3] = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
];

const P2_PANEL: [[char; 5]; 5] = [
    [' ', ' ', '1', ' ', ' '],
    [' ', '2', '3', '4', ' '],
    ['5', '6', '7', '8', '9'],
    [' ', 'A', 'B', 'C', ' '],
    [' ', ' ', 'D', ' ', ' '],
];

fn part1() -> Result<(), Box<dyn Error + 'static>> {
    let res = fs::read_to_string("resources/day02.txt")?;
    let mut index = (1, 1);
    println!("the code for part1 is: ");
    for line in res.lines() {
        for chr in line.chars() {
            match chr {
                'U' if index.0 > 0 => index.0 -= 1,
                'R' if index.1 < 2 => index.1 += 1,
                'D' if index.0 < 2 => index.0 += 1,
                'L' if index.1 > 0 => index.1 -= 1,
                _ => {}
            }
        }
        let key = P1_PANEL[index.0][index.1];
        print!("{key}");
    }
    println!();
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error + 'static>> {
    let res = fs::read_to_string("resources/day02.txt")?;
    let mut index = (1, 1);
    println!("the code for part2 is: ");
    for line in res.lines() {
        for chr in line.chars() {
            match chr {
                'U' if index.0 > 0 && P2_PANEL[index.0 - 1][index.1] != ' ' => index.0 -= 1,
                'R' if index.1 < 4 && P2_PANEL[index.0][index.1 + 1] != ' ' => index.1 += 1,
                'D' if index.0 < 4 && P2_PANEL[index.0 + 1][index.1] != ' ' => index.0 += 1,
                'L' if index.1 > 0 && P2_PANEL[index.0][index.1 - 1] != ' ' => index.1 -= 1,
                _ => {}
            }
        }
        let key = P2_PANEL[index.0][index.1];
        print!("{key}");
    }
    println!();
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + 'static>>{
    part1()?;
    part2()?;
    Ok(())
}
