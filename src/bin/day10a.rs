use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut total = 0;

    for line in input.trim().split('\n') {
        let mut stack = vec![];
        for c in line.chars() {
            if "([{<".find(c).is_some() {
                stack.push(c);
            } else if stack.pop() != Some(get(c).0) {
                total += get(c).1;
                break;
            }
        }
    }

    println!("{}", total);
}

fn get(c: char) -> (char, i32) {
    match c {
        ')' => ('(', 3),
        ']' => ('[', 57),
        '}' => ('{', 1197),
        '>' => ('<', 25137),
        _ => unreachable!(),
    }
}
