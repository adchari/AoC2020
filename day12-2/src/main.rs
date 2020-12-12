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

#[derive(Copy, Clone)]
struct Point(i32, i32);

struct Boat {
    loc: Point,
    way: Point,
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

fn right(it: i32, loc: &Point, way: &Point) -> Point {
    if it == 0 {
        return *way;
    }
    let x_o = way.0 - loc.0;
    let y_o = way.1 - loc.1;
    let x_new = y_o;
    let y_new = -x_o;
    let p = Point(x_new + loc.0, y_new + loc.1);
    right(it - 1, &loc, &p)
}

fn left(it: i32, loc: &Point, way: &Point) -> Point {
    if it == 0 {
        return *way;
    };
    let x_o = way.0 - loc.0;
    let y_o = way.1 - loc.1;
    let x_new = -y_o;
    let y_new = x_o;
    let p = Point(x_new + loc.0, y_new + loc.1);
    left(it - 1, &loc, &p)
}

fn forward(it: i32, loc: &Point, way: &Point) -> (Point, Point) {
    let delta = Point(way.0 - loc.0, way.1 - loc.1);
    let movement = Point(it * delta.0, it * delta.1);
    let loc_2 = Point(loc.0 + movement.0, loc.1 + movement.1);
    let way_2 = Point(way.0 + movement.0, way.1 + movement.1);
    (loc_2, way_2)
}

fn apply(b: Boat, inst: Vec<Instruction>) -> Boat {
    let mut boat = b;
    for it in inst {
        match it {
            Instruction::N(x) => boat.way.1 += x,
            Instruction::S(x) => boat.way.1 -= x,
            Instruction::E(x) => boat.way.0 += x,
            Instruction::W(x) => boat.way.0 -= x,
            Instruction::L(x) => boat.way = left(x, &boat.loc, &boat.way),
            Instruction::R(x) => boat.way = right(x, &boat.loc, &boat.way),
            Instruction::F(x) => {
                let (l, w) = forward(x, &boat.loc, &boat.way);
                boat.loc = l;
                boat.way = w;
            }
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
        way: Point(10, 1),
    };

    let end = apply(start, list);
    let val = manhattan_distance(Point(0, 0), end.loc);

    println!("{}", val);
}
