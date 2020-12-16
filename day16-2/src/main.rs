use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Range {
    field: String,
    low_1: usize,
    high_1: usize,
    low_2: usize,
    high_2: usize,
}

#[derive(Debug, Clone)]
struct Ticket {
    v: Vec<usize>,
}

fn parse(input: String) -> (Vec<Range>, Ticket, Vec<Ticket>) {
    let spl: Vec<&str> = input.split("\n\n").collect();
    let parameters: Vec<Range> = spl[0]
        .lines()
        .map(|s| {
            let by_colon: Vec<&str> = s.split(": ").collect();
            let field = String::from(by_colon[0]);
            let range_split: Vec<&str> = by_colon[1].split(" or ").collect();
            let range_1: Vec<&str> = range_split[0].split("-").collect();
            let range_2: Vec<&str> = range_split[1].split("-").collect();
            let low_1 = range_1[0].parse::<usize>().unwrap();
            let high_1 = range_1[1].parse::<usize>().unwrap();
            let low_2 = range_2[0].parse::<usize>().unwrap();
            let high_2 = range_2[1].parse::<usize>().unwrap();
            Range {
                field,
                low_1,
                high_1,
                low_2,
                high_2,
            }
        })
        .collect();

    let s: &str = spl[1].lines().nth(1).unwrap();

    let your: Ticket = {
        let spl: Vec<usize> = s
            .split(",")
            .map(|val| val.parse::<usize>().unwrap())
            .collect();
        Ticket { v: spl }
    };

    let split_portion: Vec<&str> = spl[2].lines().collect();
    let others: Vec<Ticket> = split_portion
        .iter()
        .skip(1)
        .map(|next| {
            let spl: Vec<usize> = next
                .split(",")
                .map(|val| val.parse::<usize>().unwrap())
                .collect();
            Ticket { v: spl }
        })
        .collect();

    (parameters, your, others)
}

fn in_range(range: &Range, val: usize) -> bool {
    if val >= range.low_1 && val <= range.high_1 {
        return true;
    }

    if val >= range.low_2 && val <= range.high_2 {
        return true;
    }

    false
}

fn filter(list: &Vec<Range>, others: Vec<Ticket>) -> Vec<Ticket> {
    others
        .into_iter()
        .filter(|tix| {
            tix.v.iter().all(|&val| {
                if list.iter().any(|r| in_range(r, val)) {
                    true
                } else {
                    false
                }
            })
        })
        .collect()
}

fn matching(tickets: Vec<Ticket>, ranges: Vec<Range>) -> Vec<String> {
    let field_count = tickets[0].v.len();
    let mut sets: Vec<HashSet<String>> = (0..field_count)
        .map(|i| {
            let mut set: HashSet<String> = HashSet::new();
            ranges.iter().for_each(|r| {
                if tickets.iter().all(|tix| in_range(r, tix.v[i])) {
                    set.insert(r.field.clone());
                }
            });
            set
        })
        .collect();

    while sets.iter().any(|s| s.len() != 1) {
        let singles: Vec<usize> = sets
            .iter()
            .enumerate()
            .filter(|(_i, x)| x.len() == 1)
            .map(|(i, _x)| i)
            .collect();

        let mut sets_new: Vec<HashSet<String>> = Vec::new();
        for (i, x) in sets.iter().enumerate() {
            if singles.contains(&i) {
                sets_new.push(x.clone());
                continue;
            }

            let set: HashSet<String> = x
                .iter()
                .filter(|&item| !singles.iter().any(|&idx| sets[idx].contains(item)))
                .map(|s| s.to_string())
                .collect();
            sets_new.push(set);
        }
        sets = sets_new;
    }

    sets.iter()
        .map(|x| {
            x.into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .flatten()
        .collect::<Vec<String>>()
}

fn answer(yours: Ticket, matching: Vec<String>) -> usize {
    let indices: Vec<usize> = matching
        .iter()
        .enumerate()
        .filter(|(_i, e)| e.starts_with("departure"))
        .map(|(i, _e)| i)
        .collect();
    indices.iter().map(|&idx| yours.v[idx]).product()
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

    let (list, yours, nearby) = parse(buffer);

    let mut new_tix = filter(&list, nearby);
    new_tix.push(yours.clone());

    let m = matching(new_tix, list);
    let val = answer(yours, m);
    println!("{}", val);
}
