use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn two_sum(list: &[i32], target: i32) -> bool {
    let mut complements: HashMap<i32, usize> = HashMap::new();
    for (i, num) in list.iter().enumerate() {
        match complements.get(num) {
            Some(_) => return true,
            None => complements.insert(target - num, i),
        };
    }
    return false;
}

fn driver(list: &Vec<i32>) -> i32 {
    for window in list.windows(26) {
        if !two_sum(&window[0..=24], window[25]) {
            return window[25];
        }
    }
    return -1;
}

fn subarray_sum(arr: &Vec<i32>, target: i32) -> (usize, usize) {
    let mut curr_sum = arr[0];
    let mut start = 0;
    let mut i = 1;
    while i <= arr.len() {
        while curr_sum > target && start < i - 1 {
            curr_sum -= arr[start];
            start += 1;
        }

        if curr_sum == target {
            return (start, i - 1);
        }

        if i < arr.len() {
            curr_sum += arr[i];
        }
        i += 1;
    }
    return (0, 0);
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

    let target: i32 = driver(&list);
    let (start, end) = subarray_sum(&list, target);
    let sub_list: Vec<i32> = list[start..=end].to_vec();

    println!(
        "{}",
        sub_list.iter().min().unwrap() + sub_list.iter().max().unwrap()
    );
}
