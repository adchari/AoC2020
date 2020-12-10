use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn dist(list: Vec<i32>) -> (i32, i32, i32) {
    let mut a = 0;
    let mut b = 0;
    let mut c = 1;
    match list[0] {
        1 => a += 1,
        2 => b += 1,
        3 => c += 1,
        _ => panic!("Difference between elements was not 1-3"),
    }

    for window in list.windows(2) {
        match window[1] - window[0] {
            1 => a += 1,
            2 => b += 1,
            3 => c += 1,
            _ => panic!("Difference between elements was not 1-3"),
        }
    }
    (a, b, c)
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

    list.sort();
    let (a, _, c): (i32, i32, i32) = dist(list);
    println!("{}", a * c);
}
