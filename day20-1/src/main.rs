use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    arr: Vec<Vec<char>>,
    edge_hashes: Vec<usize>,
}

fn hash(edge: &Vec<char>) -> usize {
    let mut hash: usize = 0;
    for i in 0..edge.len() {
        if edge[i] == '#' {
            hash += 2usize.pow(i as u32);
        }
    }
    hash
}

fn parse(tile: &str) -> Tile {
    let lines: Vec<&str> = tile.lines().collect();
    let id_str = lines[0]
        .strip_prefix("Tile ")
        .unwrap()
        .strip_suffix(":")
        .unwrap();
    let id = id_str.parse::<usize>().unwrap();

    let arr: Vec<Vec<char>> = lines[1..]
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let mut edges: Vec<Vec<char>> = Vec::new();

    let mut top = arr[0].clone();
    edges.push(top.clone());
    top.reverse();
    edges.push(top);

    let mut bot = arr[arr.len() - 1].clone();
    edges.push(bot.clone());
    bot.reverse();
    edges.push(bot);

    let mut left: Vec<char> = Vec::new();
    for i in 0..arr.len() {
        left.push(arr[i][0]);
    }
    edges.push(left.clone());
    left.reverse();
    edges.push(left);

    let mut right: Vec<char> = Vec::new();
    for i in 0..arr.len() {
        right.push(arr[i][arr[i].len() - 1]);
    }
    edges.push(right.clone());
    right.reverse();
    edges.push(right);

    let edge_hashes: Vec<usize> = edges.iter().map(|x| hash(x)).collect();

    Tile {
        id: id,
        arr: arr,
        edge_hashes: edge_hashes,
    }
}

fn puzzle(tiles: Vec<Tile>) -> Vec<usize> {
    let mut occurrences: HashMap<usize, usize> = HashMap::new();
    for tile in tiles.iter() {
        for val in tile.edge_hashes.iter() {
            let occ = occurrences.get(val).unwrap_or(&0).clone();
            occurrences.insert(*val, occ + 1);
        }
    }

    let mut ret: Vec<usize> = Vec::new();
    for tile in tiles.iter() {
        let unmatched: usize = tile
            .edge_hashes
            .iter()
            .map(|v| {
                if *occurrences.get(v).unwrap_or(&1) == 1 {
                    1
                } else {
                    0
                }
            })
            .sum();

        if unmatched == 4 {
            ret.push(tile.id);
        }
    }
    ret
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

    let split: Vec<&str> = buffer.split("\n\n").collect();
    let tiles: Vec<Tile> = split.iter().map(|s| parse(s)).collect();
    let corners = puzzle(tiles);

    println!("{}", corners.iter().product::<usize>());
}
