use std::fs;
use std::error::Error;
use std::collections::HashMap;

fn built_count(input: &str) -> Vec<HashMap<char, u32>> {
    let mut maps: Vec<HashMap<char, u32>> = vec![];
    for line in input.lines() {
        for (i, chr) in line.chars().enumerate() {
            if i >= maps.len() {
                maps.push(HashMap::new());
            }
            maps.get_mut(i).unwrap()
                .entry(chr)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
    }
    maps
}

fn part1() -> Result<(), Box<dyn Error + 'static>> {
    let res = fs::read_to_string("resources/day06.txt")?;
    print!("the signal in part 1 is: ");
    for map in built_count(&res) {
        let chr = map.iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap().0;
        print!("{chr}");
    }
    println!();
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error + 'static>> {
    let res = fs::read_to_string("resources/day06.txt")?;
    print!("the signal in part 2 is: ");
    for map in built_count(&res) {
        let chr = map.iter()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap().0;
        print!("{chr}");
    }
    println!();
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + 'static>>{
    part1()?;
    part2()?;
    Ok(())
}
