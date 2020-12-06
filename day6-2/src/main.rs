use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn count(group: &str) -> usize {
    let arr: Vec<&str> = group.split_whitespace().collect();
    let mut sets: Vec<HashSet<char>> = Vec::new();

    for it in arr.iter() {
        let mut set: HashSet<char> = HashSet::new();
        for c in it.chars() {
            set.insert(c);
        }
        sets.push(set);
    }

    let intersection: Vec<_> = sets[0]
        .iter()
        .filter(|k| sets.iter().all(|s| s.contains(k)))
        .collect();
    intersection.len()
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
