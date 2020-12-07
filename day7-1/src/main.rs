use petgraph::graph::Graph;
use petgraph::visit::Dfs;
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

/// Reverses the direction of the adjacency
/// i.e. if a posh brown bag contains 5 dim coral bags,
/// dim coral ---> posh brown with weight 5
fn adj_list(parsed: Vec<(String, Vec<(String, usize)>)>) -> HashMap<String, Vec<(String, usize)>> {
    let mut map: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    for i in parsed.iter() {
        let (container, contained) = i;
        for j in contained.iter() {
            let (key, val) = j;
            let from_map = map.get(key);
            match from_map {
                Some(v) => {
                    let mut new_list = v.clone();
                    new_list.push((container.to_string(), *val));
                    map.insert(key.to_string(), new_list);
                }
                None => {
                    let mut new_list: Vec<(String, usize)> = Vec::new();
                    new_list.push((container.to_string(), *val));
                    map.insert(key.to_string(), new_list);
                }
            }
        }
    }
    map
}

fn graph_create(
    list: HashMap<String, Vec<(String, usize)>>,
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
    let mut dfs = Dfs::new(&g, start);
    let mut count: usize = 0;
    while let Some(_) = dfs.next(&g) {
        count += 1;
    }
    count
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
    let adj = adj_list(parsed);
    let g = graph_create(adj);
    let val: usize = dfs_size(g);
    println!("{}", val - 1);
}
