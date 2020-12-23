use std::char;
use std::collections::VecDeque;

fn parse(item: &str) -> VecDeque<usize> {
    let spl: Vec<char> = item.chars().collect();
    let mut v: VecDeque<usize> = VecDeque::new();
    v.reserve_exact(1_000_000);
    for it in spl {
        let cast: usize = it.to_digit(10 as u32).unwrap() as usize;
        v.push_back(cast);
    }

    for i in 10usize..=1_000_000 {
        v.push_back(i);
    }
    v
}

fn run(cups: &mut VecDeque<usize>, iterations: usize) {
    for _ in 0..iterations {
        let mut splice_out = VecDeque::<usize>::new();

        let current: usize = cups.pop_front().unwrap();
        for _ in 0..3 {
            let pop: usize = cups.pop_front().unwrap();
            splice_out.push_back(pop);
        }
        cups.push_front(current);

        let mut dest_label: usize = current - 1;
        while splice_out.contains(&dest_label) || dest_label == 0 {
            if dest_label <= 1 {
                dest_label = 1_000_001;
            }
            dest_label -= 1;
        }

        // rotate so that dest_label is the last element,
        // then insert splice_out and rotate so that current + 1 is the next element
        let dest_idx: usize = cups.iter().position(|&x| x == dest_label).unwrap();
        cups.rotate_left(dest_idx);
        cups.rotate_left(1);
        for it in splice_out {
            cups.push_back(it);
        }

        cups.rotate_right(dest_idx + 3);
    }
}

fn answer(cups: &mut VecDeque<usize>) -> u128 {
    let curr_idx: usize = cups.iter().position(|&x| x == 1).unwrap();
    cups.rotate_left(curr_idx);
    (cups[1] as u128) * (cups[2] as u128)
}

fn main() {
    let inp: String = String::from("247819356");
    let mut cups: VecDeque<usize> = parse(&inp);
    run(&mut cups, 10_000_000);

    println!("{}", answer(&mut cups));
}
