use std::error::Error;
use md5;

const DOOR_ID: &str = "ugkcyxxp";

fn part1() -> Result<(), Box<dyn Error + 'static>> {
    let mut pass = String::with_capacity(8);
    for i in 0.. {
        let digest = md5::compute(DOOR_ID.to_owned() + &i.to_string());
        if digest[0] == 0 && digest[1] == 0 && digest[2] < 0b10000 {
            let chr = digest[2];
            pass.push_str(&format!("{chr:x}"));
            if pass.len() == 8 {
                break;
            }
        }
    }
    println!("the password for part 1 is {pass}");
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error + 'static>> {
    let mut pass = [' '; 8];
    for i in 0.. {
        let digest = md5::compute(DOOR_ID.to_owned() + &i.to_string());
        if digest[0] == 0 && digest[1] == 0 && digest[2] < 0b1000 {
            let pos = digest[2] as usize;
            if pass[pos] == ' ' {
                let chr = digest[3] >> 4;
                pass[pos] = format!("{chr:x}").chars().next().unwrap();
                if pass.iter().all(|x| *x != ' ') {
                    break;
                }
            }
        }
    }
    let pass: String = pass.iter().collect();
    println!("the password for part 2 is {pass}");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + 'static>>{
    part1()?;
    part2()?;
    Ok(())
}
