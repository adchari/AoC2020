mod point;
use crate::point::point::Point;

use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

fn parse(inp: Vec<String>) -> Vec<Point> {
    let grid: Vec<Vec<char>> = inp
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();
    let x_center = grid[0].len() / 2;
    let y_center = grid.len() / 2;
    let mut points: Vec<Point> = Vec::new();
    for (j, v) in grid.iter().enumerate() {
        for (i, c) in v.iter().enumerate() {
            if *c == '#' {
                points.push(Point::new(
                    (i as isize) - (x_center as isize),
                    (j as isize) - (y_center as isize),
                    0,
                    0,
                ));
            }
        }
    }
    points
}

fn run(start: BTreeSet<Point>, cycles: usize) -> usize {
    if cycles == 0 {
        return start.len();
    }

    let mut list: Vec<Point> = start.iter().map(|x| x.neighbors()).flatten().collect();
    for it in start.iter() {
        list.push(*it);
    }

    let mut set_new: BTreeSet<Point> = BTreeSet::new();
    for point in list.iter() {
        let active_neighbors: usize = point
            .neighbors()
            .iter()
            .map(|n| if start.contains(&n) { 1 } else { 0 })
            .sum();

        if start.contains(&point) {
            if active_neighbors == 2 || active_neighbors == 3 {
                set_new.insert(*point);
            }
        } else {
            if active_neighbors == 3 {
                set_new.insert(*point);
            }
        }
    }
    return run(set_new, cycles - 1);
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

    let parsed: BTreeSet<Point> = BTreeSet::from_iter(parse(list).into_iter());
    let val = run(parsed, 6);
    println!("{}", val);
}
