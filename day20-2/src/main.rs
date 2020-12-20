use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
enum Dir {
    Forward,
    Reverse,
}

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    arr_internal: Vec<Vec<char>>,
    top: Edge,
    bot: Edge,
    right: Edge,
    left: Edge,
}

#[derive(Debug, Clone)]
struct Edge {
    dir: Dir,
    forward_hash: usize,
    reverse_hash: usize,
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

    let mut top_str = arr[0].clone();
    let forward = hash(&top_str);
    top_str.reverse();
    let reverse = hash(&top_str);
    let top = Edge {
        dir: Dir::Forward,
        forward_hash: forward,
        reverse_hash: reverse,
    };

    let mut bot_str = arr[arr.len() - 1].clone();
    let forward = hash(&bot_str);
    bot_str.reverse();
    let reverse = hash(&bot_str);
    let bot = Edge {
        dir: Dir::Forward,
        forward_hash: forward,
        reverse_hash: reverse,
    };

    let mut left_str: Vec<char> = Vec::new();
    for i in 0..arr.len() {
        left_str.push(arr[i][0]);
    }
    let forward = hash(&left_str);
    left_str.reverse();
    let reverse = hash(&left_str);
    let left = Edge {
        dir: Dir::Forward,
        forward_hash: forward,
        reverse_hash: reverse,
    };

    let mut right_str: Vec<char> = Vec::new();
    for i in 0..arr.len() {
        right_str.push(arr[i][arr[i].len() - 1]);
    }
    let forward = hash(&right_str);
    right_str.reverse();
    let reverse = hash(&right_str);
    let right = Edge {
        dir: Dir::Forward,
        forward_hash: forward,
        reverse_hash: reverse,
    };

    let mut arr_internal: Vec<Vec<char>> = Vec::new();
    for i in 1..arr.len() - 1 {
        let mut v: Vec<char> = Vec::new();
        for j in 1..arr[i].len() - 1 {
            v.push(arr[i][j]);
        }
        arr_internal.push(v);
    }

    Tile {
        id: id,
        arr_internal: arr_internal,
        top: top,
        bot: bot,
        right: right,
        left: left,
    }
}

// fn flip(x_axis: bool, tile: Tile) -> Tile {}

// fn rotate(clockwise: Bool, tile: Tile) -> Tile {}

// fn puzzle(tiles: Vec<Tile>) -> Vec<Vec<usize>> {}

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
    println!("{:#?}", tiles);
    //let corners = puzzle(tiles);

    // println!("{}", corners.iter().product::<usize>());
}
