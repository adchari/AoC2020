use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn three_sum(list: &Vec<i32>, target: i32) -> (i32, i32, i32) {
    let mut complements: HashMap<i32, (usize, usize)> = HashMap::new();
    for (i, j) in (0..list.len()).tuple_combinations() {
        let new_target: i32 = (list[i] + list[j]) as i32;
        let new_target = target - new_target;
        complements.insert(new_target, (i, j));
    }

    for num in list.iter() {
        match complements.get(num) {
            Some(&(i, j)) => return (list[i], list[j], *num),
            None => continue,
        };
    }

    (-1, -1, -1)
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

    let (a, b, c): (i32, i32, i32) = three_sum(&list, 2020);
    println!("{}", a * b * c);
}
