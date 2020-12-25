use std::collections::HashMap;

fn mod_exp(subject: u128) -> HashMap<u128, usize> {
    let mut set: HashMap<u128, usize> = HashMap::new();
    let mut val: u128 = subject % 20201227;
    let mut idx: usize = 1;
    set.insert(val, idx);

    loop {
        val = (val * subject) % 20201227;
        if set.contains_key(&val) {
            break;
        }
        idx += 1;
        set.insert(val, idx);
    }

    set
}

fn find_subject(card_public: u128, door_public: u128) -> (u128, usize, usize) {
    let mut idx: u128 = 2;
    loop {
        let map = mod_exp(idx);
        if map.contains_key(&card_public) && map.contains_key(&door_public) {
            return (
                idx,
                *map.get(&card_public).unwrap(),
                *map.get(&door_public).unwrap(),
            );
        }
        idx += 1;
    }
}

fn loops(subject: u128, loop_size: usize) -> u128 {
    let mut val: u128 = 1;

    for _ in 0..loop_size {
        val = (val * subject) % 20201227;
    }

    val
}

fn main() {
    let card_public: u128 = 10943862;
    let door_public: u128 = 12721030;

    let (_subject, _card, door) = find_subject(card_public, door_public);

    let val = loops(card_public, door);
    println!("{}", val);
}
