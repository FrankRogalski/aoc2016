use std::fs;
use std::error::Error;
use std::collections::{VecDeque, HashSet};

fn abba_found(string: &str) -> bool {
    let mut chars: VecDeque<char> = VecDeque::with_capacity(4);
    for chr in string.chars() {
        chars.push_back(chr);
        if chars.len() == 4 {
            if chars[0] != chars[1] && chars[0] == chars[3] && chars[1] == chars[2] { // abba check
                return true;
            }
            chars.pop_front();
        }
    }
    false
}

fn part1() -> Result<(), Box<dyn Error + 'static>> {
    let res = fs::read_to_string("resources/day07.txt")?;
    let mut tls_count = 0;
    'outer: for line in res.lines() {
        let mut af = false;
        for (i, string) in line.split(|x: char| x.is_ascii_punctuation()).enumerate() {
            let abba = abba_found(string);
            if abba {
                if i % 2 == 1 {
                    continue 'outer;
                } else {
                    af = true
                }
            }
        }
        if af {
            tls_count += 1;
        }
    }
    println!("the tls count for part 1 is {tls_count}");
    Ok(())
}

fn aba_found(string: &str, reverse: bool) -> HashSet<String> {
    let mut chars: VecDeque<char> = VecDeque::with_capacity(4);
    let mut combinations: HashSet<String> = HashSet::new();
    for chr in string.chars() {
        chars.push_back(chr);
        if chars.len() == 3 {
            if chars[0] != chars[1] && chars[0] == chars[2] { // aba check
                if reverse {
                    let rev = chars.iter().collect::<String>()
                        .replace(&chars[0].to_string(), " ")
                        .replace(&chars[1].to_string(), &chars[0].to_string())
                        .replace(' ', &chars[1].to_string());
                    combinations.insert(rev);
                } else {
                    combinations.insert(chars.iter().collect());
                }
            }
            chars.pop_front();
        }
    }
    combinations
}

fn part2() -> Result<(), Box<dyn Error + 'static>> {
    let res = fs::read_to_string("resources/day07.txt")?;
    let mut ssl_count = 0;
    'outer: for line in res.lines() {
        let mut hypernet: Vec<HashSet<String>> = vec![];
        let mut supernet: Vec<HashSet<String>> = vec![];
        for (i, string) in line.split(|x: char| x.is_ascii_punctuation()).enumerate() {
            let in_hypernet = i % 2 == 1;
            let combinations = aba_found(string, in_hypernet);
            if in_hypernet {
                hypernet.push(combinations);
            } else {
                supernet.push(combinations);
            }
        }
        for hn in hypernet {
            for sn in &supernet {
                if sn.intersection(&hn).count() > 0 {
                    ssl_count += 1;
                    continue 'outer;
                }
            }
        }
    }
    println!("the ssl count for part 2 is {ssl_count}");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + 'static>>{
    part1()?;
    part2()?;
    Ok(())
}
