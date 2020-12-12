use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Instruction {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

enum Direction {
    North,
    South,
    East,
    West,
}

struct Point(i32, i32);

struct Boat {
    loc: Point,
    dir: Direction,
}

fn parse(input: String) -> Instruction {
    match input.chars().next().unwrap() {
        'N' => {
            let val = input[1..].parse::<i32>().unwrap();
            Instruction::N(val)
        }
        'S' => {
            let val = input[1..].parse::<i32>().unwrap();
            Instruction::S(val)
        }
        'E' => {
            let val = input[1..].parse::<i32>().unwrap();
            Instruction::E(val)
        }
        'W' => {
            let val = input[1..].parse::<i32>().unwrap();
            Instruction::W(val)
        }
        'L' => {
            let val = input[1..].parse::<i32>().unwrap();
            Instruction::L(val / 90)
        }
        'R' => {
            let val = input[1..].parse::<i32>().unwrap();
            Instruction::R(val / 90)
        }
        'F' => {
            let val = input[1..].parse::<i32>().unwrap();
            Instruction::F(val)
        }
        _ => Instruction::L(0),
    }
}

fn manhattan_distance(p1: Point, p2: Point) -> u32 {
    let x_delta: u32 = (p2.0 - p1.0).abs() as u32;
    let y_delta: u32 = (p2.1 - p1.1).abs() as u32;
    x_delta + y_delta
}

fn right(d: Direction, i: i32) -> Direction {
    if i == 0 {
        d
    } else {
        match d {
            Direction::North => right(Direction::East, i - 1),
            Direction::East => right(Direction::South, i - 1),
            Direction::South => right(Direction::West, i - 1),
            Direction::West => right(Direction::North, i - 1),
        }
    }
}

fn left(d: Direction, i: i32) -> Direction {
    if i == 0 {
        d
    } else {
        match d {
            Direction::North => left(Direction::West, i - 1),
            Direction::East => left(Direction::North, i - 1),
            Direction::South => left(Direction::East, i - 1),
            Direction::West => left(Direction::South, i - 1),
        }
    }
}

fn apply(b: Boat, inst: Vec<Instruction>) -> Boat {
    let mut boat = b;
    for it in inst {
        match it {
            Instruction::N(x) => {
                boat.loc.1 += x;
            }
            Instruction::S(x) => {
                boat.loc.1 -= x;
            }
            Instruction::E(x) => {
                boat.loc.0 += x;
            }
            Instruction::W(x) => {
                boat.loc.0 -= x;
            }
            Instruction::L(x) => {
                boat.dir = left(boat.dir, x);
            }
            Instruction::R(x) => {
                boat.dir = right(boat.dir, x);
            }
            Instruction::F(x) => match boat.dir {
                Direction::North => boat.loc.1 += x,
                Direction::East => boat.loc.0 += x,
                Direction::South => boat.loc.1 -= x,
                Direction::West => boat.loc.0 -= x,
            },
        }
    }
    boat
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let list: Vec<Instruction> = reader
        .lines()
        .map(|x| {
            let line = x.unwrap();
            parse(line)
        })
        .collect();

    let start: Boat = Boat {
        loc: Point(0, 0),
        dir: Direction::East,
    };

    let end = apply(start, list);
    let val = manhattan_distance(Point(0, 0), end.loc);

    println!("{}", val);
}
