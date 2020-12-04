use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn verify_year(val: &str, low: u32, high: u32) -> bool {
    if val.len() != 4 {
        return false;
    }
    if !val.chars().all(char::is_numeric) {
        return false;
    }
    let num = val.parse::<u32>().unwrap_or(0);
    if num < low || num > high {
        return false;
    }
    true
}

fn verify_hgt(val: &str) -> bool {
    let units = val.get((val.len() - 2)..).unwrap();
    let num_str = val.get(..(val.len() - 2)).unwrap();
    let num = num_str.parse::<u32>().unwrap_or(0);

    match units {
        "cm" => {
            if num < 150 || num > 193 {
                false
            } else {
                true
            }
        }
        "in" => {
            if num < 59 || num > 76 {
                false
            } else {
                true
            }
        }
        _ => false,
    }
}

fn verify_hcl(val: &str) -> bool {
    let v: Vec<_> = val.match_indices("#").collect();
    if v.len() != 1 {
        return false;
    }
    if v[0] != (0, "#") {
        return false;
    }

    if val.len() != 7 {
        return false;
    }
    let numbers = val.strip_prefix("#").unwrap();
    if !numbers.chars().all(|c| c.is_ascii_hexdigit()) {
        return false;
    }
    true
}

fn verify_ecl(val: &str) -> bool {
    if val == "amb"
        || val == "blu"
        || val == "brn"
        || val == "gry"
        || val == "grn"
        || val == "hzl"
        || val == "oth"
    {
        true
    } else {
        false
    }
}

fn verify_pid(val: &str) -> bool {
    if val.len() != 9 {
        return false;
    }
    val.chars().all(char::is_numeric)
}

fn valid(pass: &str) -> bool {
    let cont = pass.contains("byr:")
        && pass.contains("iyr:")
        && pass.contains("eyr:")
        && pass.contains("hgt:")
        && pass.contains("hcl:")
        && pass.contains("ecl:")
        && pass.contains("pid:");
    if !cont {
        return false;
    }

    let mut map: HashMap<&str, &str> = HashMap::new();
    for s in pass.split_whitespace() {
        let pair: Vec<&str> = s.split(":").collect();
        if pair.len() > 2 || pair.len() < 2 {
            return false;
        }
        map.insert(pair[0], pair[1]);
    }

    verify_year(map.get("byr").unwrap_or(&""), 1920, 2002)
        && verify_year(map.get("iyr").unwrap_or(&""), 2010, 2020)
        && verify_year(map.get("eyr").unwrap_or(&""), 2020, 2030)
        && verify_hgt(map.get("hgt").unwrap_or(&""))
        && verify_hcl(map.get("hcl").unwrap_or(&""))
        && verify_ecl(map.get("ecl").unwrap_or(&""))
        && verify_pid(map.get("pid").unwrap_or(&""))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let filename = &args[1];
    let mut file = File::open(filename).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let arr: Vec<&str> = buffer.split("\n\n").collect();
    let count: usize = arr
        .iter()
        .map(|s| match valid(s) {
            true => 1,
            false => {
                println!("{:#?}", s);
                0
            }
        })
        .sum();
    println!("{}", count);
}
