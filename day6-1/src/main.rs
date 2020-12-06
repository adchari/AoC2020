use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn count(group: &str) -> usize {
    let arr: Vec<&str> = group.split("\n").collect();
    let mut set: HashSet<char> = HashSet::new();

    for it in arr.iter() {
        for c in it.chars() {
            set.insert(c);
        }
    }
    return set.len();
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

    let val: usize = arr.iter().map(|x| count(x)).sum();

    println!("{}", val);
}
