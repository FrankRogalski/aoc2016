use std::fs;
use std::error::Error;
use regex::Regex;


fn part1() -> Result<(), Box<dyn Error + 'static>> {
    let spaces = Regex::new(r"\d+")?;
    let res = fs::read_to_string("resources/day03.txt")?;
    let mut possible_triangles = 0;
    for line in res.lines() {
        let mut sides: Vec<u32>= spaces.find_iter(line)
            .map(|x| x.as_str().parse::<u32>().expect("x should be a valid number but is: {x}"))
            .collect();
        assert_eq!(sides.len(), 3, "{line}");
        sides.sort();
        if sides[0] + sides[1] > sides[2] {
            possible_triangles += 1;
        }
    }
    println!("there are {possible_triangles} possible triangles in the list for part 1");
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error + 'static>> {
    let spaces = Regex::new(r"\d+")?;
    let res = fs::read_to_string("resources/day03.txt")?;
    let mut bucket1: Vec<u32> = vec![];
    let mut bucket2: Vec<u32> = vec![];
    let mut bucket3: Vec<u32> = vec![];
    for line in res.lines() {
        let mut sides: Vec<u32>= spaces.find_iter(line)
            .map(|x| x.as_str().parse::<u32>().expect("x should be a valid number but is: {x}"))
            .collect();
        assert_eq!(sides.len(), 3, "{line}");
        bucket3.push(sides.pop().unwrap());
        bucket2.push(sides.pop().unwrap());
        bucket1.push(sides.pop().unwrap());
    }
    let mut possible_triangles = 0;
    let mut last = [0; 3];
    let mut i = 0;
    for el in bucket1.iter().chain(bucket2.iter()).chain(bucket3.iter()) {
        last[i] = *el;
        if i == 2 {
            last.sort();
            if last[0] + last[1] > last[2] {
                possible_triangles += 1;
            }
        }
        i = (i + 1) % 3; 
    }

    println!("there are {possible_triangles} possible triangles in the list for part 2");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + 'static>>{
    part1()?;
    part2()?;
    Ok(())
}
