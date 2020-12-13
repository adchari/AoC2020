use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(inp: Vec<String>) -> Vec<Option<u128>> {
    inp[1]
        .split(",")
        .map(|x| match x {
            "x" => None,
            o => Some(o.parse::<u128>().unwrap()),
        })
        .collect()
}

fn run(busses: Vec<Option<u128>>) -> u128 {
    let trimmed = translate(busses);
    let mut x_found = solve_pair(trimmed[0].0, trimmed[0].1, trimmed[1]);
    let mut off_found = trimmed[0].1 * trimmed[1].1;

    let mut iter = trimmed.iter();
    iter.next();
    iter.next();
    for (a, n) in iter {
        x_found = solve_pair(x_found, off_found, (*a, *n));
        off_found = off_found * n;
    }
    x_found
}

// value, offset, remove Nones
// series of equations x == a mod n
fn translate(inp: Vec<Option<u128>>) -> Vec<(u128, u128)> {
    let mut list = inp
        .iter()
        .zip(0u128..)
        .filter(|&(e, _)| match e {
            None => false,
            Some(_) => true,
        })
        .map(|(e, i)| (e.unwrap(), i))
        .map(|(n, a)| if a < n { (a, n) } else { (a % n, n) })
        .map(|(a, n)| ((n - a) % n, n))
        .collect::<Vec<(u128, u128)>>();

    list.sort_by(|(_a1, n1), (_a2, n2)| {
        if n1 > n2 {
            Ordering::Less
        } else if n1 < n2 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    list
}

fn solve_pair(starting: u128, offset: u128, target: (u128, u128)) -> u128 {
    let mut val = starting;
    let (a, n) = target;
    loop {
        if val % n == a {
            return val;
        }
        val += offset;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let list: Vec<String> = reader
        .lines()
        .map(|x| {
            let line = x.unwrap();
            line
        })
        .collect();

    let ids = parse(list);

    let time = run(ids);
    println!("{}", time);
}
