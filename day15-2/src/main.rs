use std::collections::HashMap;

fn parse(inp: String) -> Vec<u128> {
    inp.split(",").map(|s| s.parse::<u128>().unwrap()).collect()
}

fn run(starting: &Vec<u128>) -> u128 {
    let mut map: HashMap<u128, usize> = HashMap::new();
    for (i, e) in starting.iter().enumerate() {
        map.insert(*e, i);
    }

    let mut latest = *starting.last().unwrap();
    let mut iteration = starting.len();
    map.remove(&latest);

    while iteration < 30_000_000 {
        let maybe_idx = map.get(&latest);
        match maybe_idx {
            Some(idx) => {
                let new_val = (iteration as u128) - 1 - (*idx as u128);
                map.insert(latest, iteration - 1);
                latest = new_val;
                iteration += 1;
            }
            None => {
                map.insert(latest, iteration - 1);
                latest = 0;
                iteration += 1;
            }
        }
    }

    latest
}

fn main() {
    let input: String = String::from("13,0,10,12,1,5,8");
    let list: Vec<u128> = parse(input);

    let val = run(&list);
    println!("{}", val);
}
