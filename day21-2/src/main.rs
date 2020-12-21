use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Food {
    ingredients: HashSet<Ingredient>,
    allergens: HashSet<Allergen>,
}
type Ingredient = String;
type Allergen = String;

fn parse(item: &str) -> Food {
    let spl: Vec<&str> = item.split(" (contains ").collect();
    let ingr: HashSet<Ingredient> = spl[0].split_whitespace().map(|s| s.to_string()).collect();
    let alle: HashSet<Allergen> = spl[1]
        .strip_suffix(")")
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    Food {
        ingredients: ingr,
        allergens: alle,
    }
}

fn match_allergens(foods: &Vec<Food>) -> HashMap<Allergen, Ingredient> {
    let mut map: HashMap<Allergen, HashSet<Ingredient>> = HashMap::new();
    for food in foods.iter() {
        for allergen in food.allergens.iter() {
            let current = map.get(allergen);
            match current {
                Some(food_set) => {
                    let intersection: HashSet<Ingredient> = food_set
                        .clone()
                        .intersection(&food.ingredients)
                        .map(|s| s.to_string())
                        .collect();
                    map.insert(allergen.to_string(), intersection);
                }
                None => {
                    map.insert(allergen.to_string(), food.ingredients.clone());
                }
            }
        }
    }

    while map.iter().any(|(_, v)| v.len() != 1) {
        let singles: Vec<Allergen> = map
            .iter()
            .filter(|(_, v)| v.len() == 1)
            .map(|(k, _)| k.to_string())
            .collect();

        let mut map_new: HashMap<Allergen, HashSet<Ingredient>> = HashMap::new();
        for (k, v) in map.iter() {
            if singles.contains(k) {
                map_new.insert(k.to_string(), v.clone());
                continue;
            }

            let set: HashSet<String> = v
                .iter()
                .filter(|&item| {
                    !singles
                        .iter()
                        .any(|idx| map.get(idx).unwrap().contains(item))
                })
                .map(|s| s.to_string())
                .collect();
            map_new.insert(k.to_string(), set);
        }
        map = map_new;
    }

    let mut ret: HashMap<Allergen, Ingredient> = HashMap::new();

    for (k, v) in map.iter() {
        let vector: Vec<&Ingredient> = v.into_iter().collect();
        ret.insert(k.to_string(), vector[0].to_string());
    }
    ret
}

fn dangerous_list(map: HashMap<Allergen, Ingredient>) -> String {
    let mut v: Vec<String> = Vec::new();
    let mut keys: Vec<Allergen> = map.keys().map(|k| k.to_string()).collect();
    keys.sort();
    for key in keys {
        v.push(map.get(&key).unwrap().to_string());
    }
    return v.join(",");
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

    let list: Vec<Food> = buffer.lines().map(|x| parse(x)).collect();

    let matched = match_allergens(&list);
    let s = dangerous_list(matched);

    println!("{}", s);
}
