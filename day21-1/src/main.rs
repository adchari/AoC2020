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

fn clean_ingredients(foods: &Vec<Food>) -> HashSet<Ingredient> {
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

    let mut clean: HashSet<Ingredient> = HashSet::new();
    for food in foods.iter() {
        for ingredient in food.ingredients.iter() {
            if !map.iter().any(|(_k, v)| v.contains(ingredient)) {
                clean.insert(ingredient.to_string());
            }
        }
    }
    clean
}

fn count(foods: Vec<Food>, ingredients: HashSet<Ingredient>) -> usize {
    foods
        .iter()
        .map(|food| {
            food.ingredients
                .iter()
                .map(|ing| if ingredients.contains(ing) { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
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

    let clean = clean_ingredients(&list);
    let count = count(list, clean);

    println!("{}", count);
}
