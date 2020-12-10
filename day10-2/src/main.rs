use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn distinct(list: Vec<i32>) -> u128 {
    let mut map: HashMap<i32, u128> = HashMap::new();
    map.insert(0, 1);
    for it in list.iter() {
        let sum = {
            let minus_one = map.get(&(it - 1)).unwrap_or(&0);
            let minus_two = map.get(&(it - 2)).unwrap_or(&0);
            let minus_three = map.get(&(it - 3)).unwrap_or(&0);
            minus_one + minus_two + minus_three
        };

        map.insert(*it, sum);
    }
    return *map.get(list.iter().max().unwrap_or(&0)).unwrap_or(&0);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut list: Vec<i32> = reader
        .lines()
        .map(|x| {
            let line = x.unwrap();
            let val = line.parse::<i32>().unwrap_or(0);
            val
        })
        .collect();
    list.push(*list.iter().max().unwrap());
    list.sort();

    let val = distinct(list);
    println!("{}", val);
}
