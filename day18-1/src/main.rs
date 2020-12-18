use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Num(i128),
}

fn unnecessary_parens(chars: &Vec<char>) -> bool {
    let mut stack: Vec<char> = Vec::new();

    if !(chars[0] == '(' && chars[chars.len() - 1] == ')') {
        return false;
    }

    stack.push('(');
    for i in 1..(chars.len() - 1) {
        if chars[i] == '(' {
            stack.push('(');
        } else if chars[i] == ')' {
            if stack.len() == 0 {
                panic!("Mismatched parens")
            } else {
                stack.pop();
            }
        }
        if stack.len() == 0 {
            return false;
        }
    }

    return true;
}

fn parse(input: &str) -> Expr {
    if let Ok(x) = input.parse::<i128>() {
        return Expr::Num(x);
    }
    // look for the last open operator
    let chars: Vec<char> = input.chars().filter(|&c| c != ' ').collect();
    let mut stack: Vec<char> = Vec::new();
    let mut idx = 0;
    for (i, &c) in chars.iter().enumerate() {
        if c == '+' || c == '*' {
            if stack.len() == 0 {
                idx = i;
            }
        } else if c == '(' {
            stack.push('(');
        } else if c == ')' {
            if stack.len() == 0 {
                panic!("Mismatched parens")
            } else {
                stack.pop();
            }
        }
    }

    let mut before: Vec<char> = chars[..idx].to_vec();
    let mut after: Vec<char> = chars[idx + 1..].to_vec();
    if unnecessary_parens(&before) {
        before = before[1..before.len() - 1].to_vec();
    }

    if unnecessary_parens(&after) {
        after = after[1..after.len() - 1].to_vec();
    }

    let before: String = before.iter().collect();
    let after: String = after.iter().collect();
    let solved_before: Expr = parse(&before);
    let solved_after: Expr = parse(&after);
    match chars[idx] {
        '+' => {
            return Expr::Add(Box::new(solved_before), Box::new(solved_after));
        }
        '*' => {
            return Expr::Mul(Box::new(solved_before), Box::new(solved_after));
        }
        _ => panic!("Something is wrong"),
    }
}

fn eval(expr: Expr) -> i128 {
    match expr {
        Expr::Num(x) => x,
        Expr::Add(a, b) => eval(*a) + eval(*b),
        Expr::Mul(a, b) => eval(*a) * eval(*b),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let val: i128 = reader
        .lines()
        .map(|x| {
            let line = x.unwrap();
            parse(&line)
        })
        .map(|e| eval(e))
        .sum();

    println!("{}", val);
}
