use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[macro_use]
extern crate scan_fmt;

fn scan_line(line: &str) -> (usize, usize, char, String) {
    let (low, high, c, pass): (usize, usize, char, String) =
        scan_fmt!(line, "{}-{} {}: {}", usize, usize, char, String).unwrap();
    return (low, high, c, pass);
}

fn verify(pass_data: (usize, usize, char, String)) -> bool {
    let (low, high, c, pass) = pass_data;
    let mut count: usize = 0;
    for s in pass.chars() {
        if s == c {
            count = count + 1;
        }
    }
    return count >= low && count <= high;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let count: i32 = reader
        .lines()
        .map(|x| {
            let line = x.unwrap();
            match verify(scan_line(&line)) {
                true => 1,
                false => 0,
            }
        })
        .sum();

    println!("{}", count);
}
