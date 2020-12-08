use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug)]
enum Op {
    Nop(i32),
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
        "nop" => {
            let val = split[1].parse::<i32>().unwrap();
            Op::Nop(val)
        }
        _ => Op::Jmp(0),
    }
}

fn runner(inst: &Vec<Op>) -> (bool, i32) {
    let mut idx: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();
    let mut accumulator: i32 = 0;
    while !visited.contains(&idx) {
        if (idx as usize) == inst.len() {
            return (true, accumulator);
        }

        visited.insert(idx);
        match inst[idx as usize] {
            Op::Nop(_) => {
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
    return (false, accumulator);
}

fn change_one(inst: Vec<Op>) -> i32 {
    let mut mutable = inst.clone();
    let mut idx: usize = 0;
    while idx < inst.len() {
        match inst[idx] {
            Op::Acc(_) => {
                idx += 1;
                continue;
            }
            Op::Jmp(x) => {
                mutable[idx] = Op::Nop(x);
                let (worked, val) = runner(&mutable);
                if worked {
                    return val;
                }
                mutable[idx] = Op::Jmp(x);
                idx += 1;
            }
            Op::Nop(x) => {
                mutable[idx] = Op::Jmp(x);
                let (worked, val) = runner(&mutable);
                if worked {
                    return val;
                }
                mutable[idx] = Op::Nop(x);
                idx += 1;
            }
        }
    }
    return -1;
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

    let val: i32 = change_one(parsed);
    println!("{}", val);
}
