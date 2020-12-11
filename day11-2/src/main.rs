use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn eq(arr1: &Vec<Vec<char>>, arr2: &Vec<Vec<char>>) -> bool {
    for (i, e) in arr1.iter().enumerate() {
        for (j, k) in e.iter().enumerate() {
            if *k != arr2[i][j] {
                return false;
            }
        }
    }
    true
}

fn count_all_occupied(arr: &Vec<Vec<char>>) -> u128 {
    arr.iter()
        .flatten()
        .map(|x| if *x == '#' { 1 } else { 0 })
        .sum()
}

fn num_occupied_seats(
    arr: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    x_size: usize,
    y_size: usize,
) -> u8 {
    let dir: &[(i32, i32)] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut count: u8 = 0;

    for (a, b) in dir.iter() {
        let mut x = i as i32;
        let mut y = j as i32;
        loop {
            x += a;
            y += b;
            if x < 0 || x >= (x_size as i32) || y < 0 || y >= (y_size as i32) {
                break;
            }

            match arr[x as usize][y as usize] {
                '#' => {
                    count += 1;
                    break;
                }
                'L' => {
                    break;
                }
                _ => {}
            }
        }
    }
    count
}

fn change_seat(seat: char, occupied: u8) -> char {
    match seat {
        'L' => {
            if occupied == 0 {
                '#'
            } else {
                'L'
            }
        }
        '#' => {
            if occupied >= 5 {
                'L'
            } else {
                '#'
            }
        }
        o => o,
    }
}

fn step(arr: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_arr = arr.clone();
    let x_size = arr.len();
    let y_size = arr[0].len();

    for (i, e) in arr.iter().enumerate() {
        for (j, k) in e.iter().enumerate() {
            match *k {
                '.' => {
                    new_arr[i][j] = '.';
                }
                a => {
                    new_arr[i][j] = change_seat(a, num_occupied_seats(arr, i, j, x_size, y_size));
                }
            }
        }
    }
    new_arr
}

fn steady_state(arr: Vec<Vec<char>>) -> u128 {
    let mut before = arr.clone();

    loop {
        let after = step(&before);
        if eq(&before, &after) {
            break;
        }
        before = after.clone();
    }

    count_all_occupied(&before)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let list: Vec<Vec<char>> = reader
        .lines()
        .map(|x| {
            let line = x.unwrap();
            let val: Vec<char> = line.chars().collect();
            val
        })
        .collect();

    let val = steady_state(list);
    println!("{}", val);
}
