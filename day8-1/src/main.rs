use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Op {
    Nop,
    Acc(i32),
    Jmp(i32),
}

fn parser(line: String) -> Op {
    let split: Vec<&str> = line.split_whitespace().collect();
    match split[0] {
        "acc" => {
            let val = split[1].parse::<i32>().unwrap();
            Op::Acc(val)
        }
        "jmp" => {
            let val = split[1].parse::<i32>().unwrap();
            Op::Jmp(val)
        }
        _ => Op::Nop,
    }
}

fn runner(inst: Vec<Op>) -> i32 {
    let mut idx: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();
    let mut accumulator: i32 = 0;
    while !visited.contains(&idx) && (idx as usize) < inst.len() && idx >= 0 {
        visited.insert(idx);
        match inst[idx as usize] {
            Op::Nop => {
                idx += 1;
            }
            Op::Acc(inc) => {
                accumulator += inc;
                idx += 1;
            }
            Op::Jmp(off) => {
                idx += off;
            }
        }
    }
    return accumulator;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let parsed: Vec<Op> = reader
        .lines()
        .map(|x| {
            let line = x.unwrap();
            parser(line)
        })
        .collect();

    let val: i32 = runner(parsed);
    println!("{}", val);
}
