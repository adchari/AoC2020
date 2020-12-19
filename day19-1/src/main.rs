use itertools::Itertools;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
enum Rule {
    Char {
        idx: usize,
        val: char,
    },
    Other {
        idx: usize,
        val: Vec<Vec<Box<Rule>>>,
    },
}

fn parse(rules: Vec<&str>) -> Rule {
    let mut rule_strings: BTreeMap<usize, String> = BTreeMap::new();
    for rule in rules {
        let spl: Vec<&str> = rule.split(": ").collect();
        let idx = spl[0].parse::<usize>().unwrap();
        let rest = spl[1].to_string();
        rule_strings.insert(idx, rest);
    }
    let mut found_rules: BTreeMap<usize, Rule> = BTreeMap::new();
    recursive_parse(&rule_strings, 0, &mut found_rules)
}

fn recursive_parse(
    rules: &BTreeMap<usize, String>,
    idx: usize,
    found_rules: &mut BTreeMap<usize, Rule>,
) -> Rule {
    if let Some(r) = found_rules.get(&idx) {
        return r.clone();
    }

    let s = rules.get(&idx).unwrap();
    if s.starts_with('"') {
        let c = s.chars().nth(1).unwrap();
        let rule = Rule::Char { idx: idx, val: c };
        found_rules.insert(idx, rule.clone());
        return rule;
    }

    let options: Vec<&str> = s.split(" | ").collect();
    let rule_arr: Vec<Vec<usize>> = options
        .iter()
        .map(|x| {
            x.split_whitespace()
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    let real_rules: Vec<Vec<Box<Rule>>> = rule_arr
        .iter()
        .map(|list| {
            list.iter()
                .map(|u| Box::new(recursive_parse(rules, *u, found_rules)))
                .collect::<Vec<Box<Rule>>>()
        })
        .collect();

    let rule = Rule::Other {
        idx: idx,
        val: real_rules,
    };
    found_rules.insert(idx, rule.clone());
    return rule;
}

fn eval(rule: Rule) -> HashSet<String> {
    let mut sets: BTreeMap<usize, HashSet<String>> = BTreeMap::new();
    recursive_eval(rule, &mut sets)
}

fn recursive_eval(rule: Rule, sets: &mut BTreeMap<usize, HashSet<String>>) -> HashSet<String> {
    let id: usize = match rule {
        Rule::Char { idx: i, val: _ } => i,
        Rule::Other { idx: i, val: _ } => i,
    };

    if let Some(s) = sets.get(&id) {
        return s.clone();
    }

    match rule {
        Rule::Char { idx: i, val: c } => {
            let mut set: HashSet<String> = HashSet::new();
            let mut string: String = String::from("");
            string.push(c);
            set.insert(string);
            sets.insert(i, set.clone());
            return set;
        }
        Rule::Other { idx: i, val: list } => {
            let arr: Vec<Vec<HashSet<String>>> = list
                .iter()
                .map(|list2| {
                    list2
                        .iter()
                        .map(|b| recursive_eval(*b.clone(), sets))
                        .collect::<Vec<HashSet<String>>>()
                })
                .collect();

            let set_list: Vec<HashSet<String>> = arr
                .iter()
                .map(|list2| {
                    let mut paired_sets: HashSet<String> = HashSet::new();
                    let in_arr: Vec<Vec<String>> = list2
                        .iter()
                        .map(|s| s.iter().map(|x| x.to_string()).collect::<Vec<String>>())
                        .collect();
                    for smol_vec in in_arr.iter().map(|i| i.iter()).multi_cartesian_product() {
                        let mut smol_str = String::from("");
                        for s in smol_vec {
                            smol_str.push_str(&s);
                        }
                        paired_sets.insert(smol_str);
                    }
                    paired_sets
                })
                .collect();

            let mut ret: HashSet<String> = HashSet::new();
            for s in set_list {
                for it in s {
                    ret.insert(it);
                }
            }
            sets.insert(i, ret.clone());
            return ret;
        }
    }
}

fn check(inp: &str, set: &HashSet<String>) -> bool {
    set.contains(inp)
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

    let rule = parse(split[0].lines().collect());
    let set = eval(rule);
    let count: u128 = split[1]
        .lines()
        .map(|x| match check(x, &set) {
            true => 1,
            false => 0,
        })
        .sum();

    println!("{}", count);
}
