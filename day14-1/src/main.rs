use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Instruction {
    Mask(String),
    Mem(u64, u64),
}

fn parse(inp: String) -> Instruction {
    if inp.starts_with("mask") {
        let split: Vec<_> = inp.split(" = ").collect();
        Instruction::Mask(split[1].to_string())
    } else {
        let split: Vec<_> = inp.split(" = ").collect();
        let strip1 = split[0].strip_prefix("mem[").unwrap();
        let strip2 = strip1.strip_suffix("]").unwrap();
        let loc: u64 = strip2.parse().unwrap();
        let val: u64 = split[1].parse().unwrap();
        Instruction::Mem(loc, val)
    }
}

fn mask(val: u64, mask: &str) -> u64 {
    let mut in_val = val;
    let mut new_val = 0;
    let mask_chars: Vec<_> = mask.chars().rev().collect();
    let base: u64 = 2;

    for i in 0u32..36 {
        if mask_chars[i as usize] == '1' {
            new_val += base.pow(i);
        } else if mask_chars[i as usize] == 'X' && in_val % 2 == 1 {
            new_val += base.pow(i);
        }
        in_val /= 2;
    }
    new_val
}

fn run(inst: Vec<Instruction>) -> BTreeMap<u64, u64> {
    let mut m: String = String::from("");
    let mut map: BTreeMap<u64, u64> = BTreeMap::new();
    for it in inst {
        match it {
            Instruction::Mask(s) => {
                m = s;
            }
            Instruction::Mem(loc, val) => {
                map.insert(loc, mask(val, &m));
            }
        }
    }
    map
}

fn sum(memory: BTreeMap<u64, u64>) -> u64 {
    let mut sum = 0;
    for (_, val) in &memory {
        sum += val;
    }
    sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let list: Vec<Instruction> = reader
        .lines()
        .map(|x| {
            let line = x.unwrap();
            parse(line)
        })
        .collect();

    let map = run(list);
    let val = sum(map);
    println!("{}", val);
}
