use std::char;
use std::collections::VecDeque;

fn parse(item: &str) -> VecDeque<usize> {
    let spl: Vec<char> = item.chars().collect();
    let mut v: VecDeque<usize> = VecDeque::new();
    for it in spl {
        let cast: usize = it.to_digit(10 as u32).unwrap() as usize;
        v.push_back(cast);
    }
    v
}

fn run(cups: VecDeque<usize>, iterations: usize) -> VecDeque<usize> {
    if iterations == 0 {
        return cups;
    }

    let mut cup_copy = cups.clone();
    let mut splice_out = VecDeque::<usize>::new();

    let current: usize = cup_copy.pop_front().unwrap();
    for _ in 0..3 {
        let pop: usize = cup_copy.pop_front().unwrap();
        splice_out.push_back(pop);
    }
    cup_copy.push_front(current);

    let mut dest_label: usize = current - 1;
    while !cup_copy.contains(&dest_label) {
        if dest_label <= 1 {
            dest_label = 10;
        }
        dest_label -= 1;
    }

    // rotate so that dest_label is the last element,
    // then insert splice_out and rotate so that current + 1 is the next element
    let dest_idx: usize = cup_copy.iter().position(|&x| x == dest_label).unwrap();
    cup_copy.rotate_left(dest_idx);
    cup_copy.rotate_left(1);
    for it in splice_out {
        cup_copy.push_back(it);
    }

    let curr_idx: usize = cup_copy.iter().position(|&x| x == current).unwrap();
    cup_copy.rotate_left(curr_idx);
    cup_copy.rotate_left(1);
    run(cup_copy, iterations - 1)
}

fn stringify(cups: &mut VecDeque<usize>) -> String {
    let curr_idx: usize = cups.iter().position(|&x| x == 1).unwrap();
    cups.rotate_left(curr_idx);
    let mut s: String = String::new();
    for i in 1..cups.len() {
        let c: char = char::from_digit(cups[i] as u32, 10).unwrap();
        s.push(c);
    }
    s
}

fn main() {
    let inp: String = String::from("247819356");
    let cups: VecDeque<usize> = parse(&inp);
    let mut cups2 = run(cups, 100);

    println!("{}", stringify(&mut cups2));
}
