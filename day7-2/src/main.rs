use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::DfsPostOrder;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Takes a line of input and returns the bag color
/// and a list of other bags (and amounts) that fit within it
fn parser(line: &str) -> (String, Vec<(String, usize)>) {
    let first_split: Vec<&str> = line.split(" contain ").collect();
    assert_eq!(first_split.len(), 2);
    let identifier: String = first_split[0]
        .split_whitespace()
        .take(2)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    if first_split[1] == "no other bags." {
        return (identifier, Vec::new());
    }

    let rest: Vec<(String, usize)> = first_split[1]
        .split(", ")
        .map(|s| {
            let split: Vec<&str> = s.split_whitespace().collect();
            let val = split[0].parse::<usize>().unwrap();
            let color = split[1..]
                .iter()
                .take(2)
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            (color, val)
        })
        .collect();
    (identifier, rest)
}

fn graph_create(
    list: Vec<(String, Vec<(String, usize)>)>,
) -> Graph<String, usize, petgraph::Directed> {
    let mut graph: Graph<String, usize, petgraph::Directed> = Graph::new();
    let mut map: HashMap<String, _> = HashMap::new();

    for (k, v) in list {
        for (it, weight) in v {
            match map.get(&k) {
                Some(_) => {}
                None => {
                    let idx = graph.add_node(k.to_string());
                    map.insert(k.to_string(), idx);
                }
            }
            match map.get(&it) {
                Some(_) => {}
                None => {
                    let idx = graph.add_node(it.to_string());
                    map.insert(it.to_string(), idx);
                }
            }
            graph.add_edge(*map.get(&k).unwrap(), *map.get(&it).unwrap(), weight);
        }
    }
    graph
}

fn dfs_size(g: Graph<String, usize, petgraph::Directed>) -> usize {
    let start = g
        .node_indices()
        .find(|x| g.node_weight(*x).unwrap() == "shiny gold")
        .unwrap();
    let mut dfs = DfsPostOrder::new(&g, start);
    let mut map: HashMap<NodeIndex, usize> = HashMap::new();
    while let Some(v) = dfs.next(&g) {
        let mut count = 1;
        for neighbor in g.neighbors(v) {
            let weight: usize = g.edges_connecting(v, neighbor).map(|e| e.weight()).sum();
            count += map.get(&neighbor).unwrap_or(&1) * weight;
        }
        map.insert(v, count);
    }
    *map.get(&start).unwrap_or(&1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let parsed: Vec<_> = reader
        .lines()
        .map(|x| {
            let line = x.unwrap();
            parser(&line)
        })
        .collect();
    let g = graph_create(parsed);
    let val: usize = dfs_size(g);
    println!("{}", val - 1);
}
