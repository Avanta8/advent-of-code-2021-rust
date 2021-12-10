use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut scores = vec![];
    for line in input.trim().split('\n') {
        let mut stack = vec![];
        let mut res = true;
        for c in line.chars() {
            if "([{<".find(c).is_some() {
                stack.push(c);
            } else if stack.pop() != Some(neg(c)) {
                res = false;
                break;
            }
        }
        if res {
            scores.push(stack.into_iter().rev().fold(0, |acc, v| acc * 5 + get(v)));
        }
    }

    scores.sort_unstable();
    println!("{}", scores[scores.len() / 2]);
}

fn neg(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => unreachable!(),
    }
}

fn get(c: char) -> i64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!(),
    }
}
