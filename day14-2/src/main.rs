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

fn mask(val: u64, mask: &str) -> Vec<u64> {
    let mask_chars: Vec<_> = mask.chars().rev().collect();
    mask_helper(val, &mask_chars, 0)
}

fn mask_helper(val: u64, mask_rev: &Vec<char>, idx: usize) -> Vec<u64> {
    if idx == mask_rev.len() {
        return vec![0];
    }

    let rest = mask_helper(val / 2, mask_rev, idx + 1);
    let mut new_vec: Vec<u64> = Vec::new();
    let bitmask_bit = mask_rev[idx];
    let val_bit = val % 2;
    let base: u64 = 2;
    for it in rest {
        if bitmask_bit == '0' {
            if val_bit == 1 {
                new_vec.push(it + base.pow(idx as u32));
            } else {
                new_vec.push(it);
            }
        } else if bitmask_bit == '1' {
            new_vec.push(it + base.pow(idx as u32));
        } else {
            new_vec.push(it + base.pow(idx as u32));
            new_vec.push(it);
        }
    }
    new_vec
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
                for idx in mask(loc, &m) {
                    map.insert(idx, val);
                }
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
