use std::fs;
use std::error::Error;
use regex::Regex;

type Order = Vec<(u8, char)>;

fn update_if_possible(chr: char, order: &mut Order) -> bool {
    for item in order.iter_mut() {
        if item.1 == chr {
            item.0 += 1;
            return true;
        }
    }
    false
}

fn update_or_insert(chr: char, order: &mut Order) {
    if !update_if_possible(chr, order) {
        order.push((1, chr));
    }
}

fn parse_line(line: &str) -> (String, String, Order) {
    let mut digits: String = String::new();
    let mut check: String = String::new();
    let mut order: Vec<(u8, char)> = vec![];
    for chr in line.chars() {
        if chr.is_numeric(){
            digits.push(chr);
        } else if chr.is_alphabetic() {
            if digits.is_empty() {
                update_or_insert(chr, &mut order)
            } else {
                check.push(chr);
            }
        } 
    }
    order.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    order.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    (digits, check, order)
}


fn get_sector_id_if_valid(digits: &str, check: &str, order: Order) -> Option<u32> {
    let mut real = true;
    for (c1, c2) in check.chars().zip(order.iter().map(|x| x.1)) {
        if c1 != c2 {
            real = false;
            break;
        }
    }
    if real {
        return digits.parse::<u32>().ok();
    }
    None
}

fn part1() -> Result<(), Box<dyn Error + 'static>> {
    let res = fs::read_to_string("resources/day04.txt")?;
    let mut sum = 0;
    for line in res.lines() {
        let (digits, check, order) = parse_line(line);
        if let Some(sector_id) = get_sector_id_if_valid(&digits, &check, order) {
            sum += sector_id;
        }
    }
    println!("the sum of the sector ids for part 1 is {sum}");
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error + 'static>> {
    let res = fs::read_to_string("resources/day04.txt")?;
    for line in res.lines() {
        let (name, sector_id) = line.rsplit_once('-').expect("there should always be a -");
        let sector_id: u32 = sector_id.split('[').next().unwrap().parse()?;
        
        let mut decrypted = String::with_capacity(name.len());
        for chr in name.chars() {
            if chr == '-' {
                decrypted.push(' ');
            } else {
                let new = char::from_u32(((chr as u32) - 97 + sector_id) % 26 + 97).expect("my math to only result in valid ascii (if ascii alpha is given)");
                decrypted.push(new);
            }
        }
        println!("decrypted {name} and got {decrypted} - the sector id was {sector_id}");
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + 'static>>{
    part1()?;
    part2()?;
    Ok(())
}
