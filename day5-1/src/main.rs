use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn row(s: &str, low: usize, high: usize) -> usize {
    if s.len() == 0 {
        return low;
    }
    let mid = low + ((high - low) / 2);
    match s.chars().nth(0).unwrap() {
        'F' => row(&s[1..], low, mid),
        'B' => row(&s[1..], mid + 1, high),
        _ => 128,
    }
}

fn col(s: &str, low: usize, high: usize) -> usize {
    if s.len() == 0 {
        return low;
    }
    let mid = low + ((high - low) / 2);
    match s.chars().nth(0).unwrap() {
        'L' => col(&s[1..], low, mid),
        'R' => col(&s[1..], mid + 1, high),
        _ => 8,
    }
}

fn seat_id(row: usize, col: usize) -> usize {
    row * 8 + col
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let val: usize = reader
        .lines()
        .map(|x| {
            let line = x.unwrap();
            let row_code = &line[..7];
            let col_code = &line[7..];
            seat_id(row(row_code, 0, 127), col(col_code, 0, 7))
        })
        .max()
        .unwrap_or(0);

    println!("{}", val);
}
