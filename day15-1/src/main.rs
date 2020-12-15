fn parse(inp: String) -> Vec<u128> {
    inp.split(",").map(|s| s.parse::<u128>().unwrap()).collect()
}

fn last_spoken(list: &Vec<u128>, target: u128) -> Option<u128> {
    for idx in (0..(list.len() - 1)).rev() {
        if list[idx] == target {
            return Some(idx as u128);
        }
    }
    None
}

fn run(starting: &Vec<u128>) -> u128 {
    let mut vector = starting.clone();

    while vector.len() != 2020 {
        let last = vector.last().unwrap();
        let prior = last_spoken(&vector, *last);
        match prior {
            Some(i) => vector.push((vector.len() as u128) - 1 - i),
            None => vector.push(0),
        }
    }

    *vector.last().unwrap()
}

fn main() {
    let input: String = String::from("13,0,10,12,1,5,8");
    let list: Vec<u128> = parse(input);

    let val = run(&list);
    println!("{}", val);
}
