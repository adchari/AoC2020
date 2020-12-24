use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

type Inst = Vec<Dir>;
type HexLoc = (isize, isize);

fn parse(s: &str) -> Inst {
    let mut v: Inst = Vec::new();
    let mut iter = s.chars();
    while let Some(c) = iter.next() {
        match c {
            'e' => v.push(Dir::E),
            'w' => v.push(Dir::W),
            'n' => {
                if let Some(d) = iter.next() {
                    match d {
                        'e' => v.push(Dir::NE),
                        'w' => v.push(Dir::NW),
                        _ => panic!("Non directional char"),
                    }
                } else {
                    panic!("Nothing after N");
                }
            }
            's' => {
                if let Some(d) = iter.next() {
                    match d {
                        'e' => v.push(Dir::SE),
                        'w' => v.push(Dir::SW),
                        _ => panic!("Non directional char"),
                    }
                } else {
                    panic!("Nothing after S");
                }
            }
            _ => panic!("Non directional char"),
        }
    }
    v
}

fn mover(loc: HexLoc, dir: &Dir) -> HexLoc {
    let (x, y) = loc;
    match dir {
        Dir::E => (x + 2, y),
        Dir::W => (x - 2, y),
        Dir::SE => (x + 1, y - 1),
        Dir::SW => (x - 1, y - 1),
        Dir::NE => (x + 1, y + 1),
        Dir::NW => (x - 1, y + 1),
    }
}

fn locate(inst: &Inst) -> HexLoc {
    let mut loc: HexLoc = (0, 0);
    for it in inst {
        loc = mover(loc, it);
    }
    loc
}

fn generate(locations: Vec<Inst>) -> HashSet<HexLoc> {
    let mut black: HashSet<HexLoc> = HashSet::new();
    let tiles: Vec<HexLoc> = locations.iter().map(|x| locate(x)).collect();
    for tile in tiles {
        if black.contains(&tile) {
            black.remove(&tile);
        } else {
            black.insert(tile);
        }
    }
    black
}

fn neighbors(s: &HexLoc) -> Vec<HexLoc> {
    let mut v: Vec<HexLoc> = Vec::new();
    let (x, y) = *s;
    v.push((x + 2, y));
    v.push((x - 2, y));
    v.push((x + 1, y + 1));
    v.push((x - 1, y + 1));
    v.push((x + 1, y - 1));
    v.push((x - 1, y - 1));
    v
}

fn black_adj(loc: &HexLoc, blacks: &HashSet<HexLoc>) -> usize {
    neighbors(loc)
        .iter()
        .map(|x| if blacks.contains(x) { 1 } else { 0 })
        .sum()
}

fn iteration(black: HashSet<HexLoc>) -> HashSet<HexLoc> {
    let assess: HashSet<HexLoc> = black
        .iter()
        .map(|h| neighbors(h).into_iter())
        .flatten()
        .collect();

    let iterate: HashSet<HexLoc> = assess.union(&black).map(|(a, b)| (*a, *b)).collect();

    let mut new_black: HashSet<HexLoc> = HashSet::new();
    for loc in iterate {
        if black.contains(&loc) {
            let adj = black_adj(&loc, &black);
            if adj == 1 || adj == 2 {
                new_black.insert(loc);
            }
        } else {
            let adj = black_adj(&loc, &black);
            if adj == 2 {
                new_black.insert(loc);
            }
        }
    }
    new_black
}

fn run(starting: HashSet<HexLoc>) -> usize {
    let mut set = starting;
    for _ in 0..100 {
        set = iteration(set);
    }
    set.len()
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

    let list: Vec<Inst> = buffer.lines().map(|x| parse(x)).collect();
    let start = generate(list);
    let count = run(start);
    println!("{}", count);
}
