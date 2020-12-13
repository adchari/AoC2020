use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(inp: Vec<String>) -> (u32, Vec<u32>) {
    let stamp = inp[0].parse::<u32>().unwrap();
    let spl: Vec<&str> = inp[1].split(",").collect();
    let filtered: Vec<_> = spl.iter().filter(|x| *x != &"x").collect();
    let mapped: Vec<u32> = filtered.iter().map(|x| x.parse::<u32>().unwrap()).collect();
    (stamp, mapped)
}

fn run(timestamp: u32, busses: Vec<u32>) -> (u32, u32) {
    let mut time = timestamp;
    loop {
        for bus in &busses {
            match time % bus {
                0 => return (time - timestamp, *bus),
                _ => continue,
            }
        }
        time += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let list: Vec<String> = reader
        .lines()
        .map(|x| {
            let line = x.unwrap();
            line
        })
        .collect();

    let (timestamp, ids) = parse(list);

    let (wait, id) = run(timestamp, ids);
    println!("{}", wait * id);
}
