use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn two_sum(list: &Vec<i32>, target: i32) -> (i32, i32) {
    let mut complements: HashMap<i32, usize> = HashMap::new();
    for (i, num) in list.iter().enumerate() {
        match complements.get(num) {
            Some(&idx) => return (list[idx], *num),
            None => complements.insert(target - num, i),
        };
    }
    (-1, -1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let list: Vec<i32> = reader
        .lines()
        .map(|x| {
            let line = x.unwrap();
            let val = line.parse::<i32>().unwrap_or(0);
            val
        })
        .collect();

    let (a, b): (i32, i32) = two_sum(&list, 2020);
    println!("{}", a * b);
}
