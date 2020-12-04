use std::env;
use std::fs::File;
use std::io::prelude::*;

fn valid(pass: &str) -> bool {
    pass.contains("byr:")
        && pass.contains("iyr:")
        && pass.contains("eyr:")
        && pass.contains("hgt:")
        && pass.contains("hcl:")
        && pass.contains("ecl:")
        && pass.contains("pid:")
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
            false => 0,
        })
        .sum();
    println!("{}", count);
}
