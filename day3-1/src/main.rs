use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn count(arr: Vec<Vec<bool>>, down: usize, right: usize) -> usize {
    let width = arr[0].len();
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut count: usize = 0;
    while y < arr.len() {
        if arr[y][x % width] {
            count = count + 1;
        }
        y = y + down;
        x = x + right;
    }
    return count;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let arr: Vec<Vec<bool>> = reader
        .lines()
        .map(|x| {
            let line = x.unwrap();
            let arr_in: Vec<bool> = line
                .chars()
                .map(|c| match c {
                    '#' => true,
                    _ => false,
                })
                .collect();
            arr_in
        })
        .collect();

    println!("{}", count(arr, 1, 3));
}
