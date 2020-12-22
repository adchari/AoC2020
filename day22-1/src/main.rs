use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn parse(item: &str) -> VecDeque<usize> {
    let spl: Vec<&str> = item.lines().collect();
    let mut v: VecDeque<usize> = VecDeque::new();
    for i in 1..spl.len() {
        let val: usize = spl[i].parse::<usize>().unwrap();
        v.push_back(val);
    }
    v
}

fn run(player_1: &mut VecDeque<usize>, player_2: &mut VecDeque<usize>) -> usize {
    if player_1.len() == 0 {
        return score(player_2);
    }

    if player_2.len() == 0 {
        return score(player_1);
    }

    let p1 = player_1.pop_front().unwrap();
    let p2 = player_2.pop_front().unwrap();

    if p1 > p2 {
        player_1.push_back(p1);
        player_1.push_back(p2);
        run(player_1, player_2)
    } else {
        player_2.push_back(p2);
        player_2.push_back(p1);
        run(player_1, player_2)
    }
}

fn score(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .enumerate()
        .map(|(i, e)| e * (deck.len() - i))
        .sum()
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
    let spl: Vec<&str> = buffer.split("\n\n").collect();
    let mut player_1 = parse(spl[0]);
    let mut player_2 = parse(spl[1]);

    println!("{}", run(&mut player_1, &mut player_2));
}
