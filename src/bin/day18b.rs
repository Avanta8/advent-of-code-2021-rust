use std::io::Read;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Item {
    Open,
    Close,
    Value(u8),
}

fn parse_input() -> Vec<Vec<Item>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    input
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .filter_map(|c| match c {
                    '0'..='9' => Some(Item::Value(c.to_digit(10).unwrap() as u8)),
                    '[' => Some(Item::Open),
                    ']' => Some(Item::Close),
                    _ => None,
                })
                .collect()
        })
        .collect()
}

fn split(seq: &[Item]) -> (Vec<Item>, bool) {
    let mut new_seq = vec![];
    let mut changed = false;
    for &item in seq {
        if let Item::Value(v) = item {
            if !changed && v >= 10 {
                new_seq.push(Item::Open);
                new_seq.push(Item::Value(v / 2));
                new_seq.push(Item::Value(v - v / 2));
                new_seq.push(Item::Close);
                changed = true;
                continue;
            }
        }
        new_seq.push(item);
    }
    (new_seq, changed)
}

fn explode(seq: &[Item]) -> (Vec<Item>, bool) {
    let mut new_seq = vec![];
    let mut depth = 0;
    let mut to_add = None;
    let mut seq_iter = seq.iter();
    let mut changed = false;

    while let Some(&item) = seq_iter.next() {
        match item {
            Item::Open => {
                depth += 1;
                new_seq.push(item);
            }
            Item::Close => {
                depth -= 1;
                new_seq.push(item);
            }
            Item::Value(v) => {
                if let Some(a) = to_add {
                    new_seq.push(Item::Value(v + a));
                    break;
                } else if depth == 5 {
                    changed = true;
                    new_seq.pop(); // remove opening bracket
                    depth -= 1;
                    let mut stack = vec![];
                    while let Some(p) = new_seq.pop() {
                        if let Item::Value(n) = p {
                            new_seq.push(Item::Value(n + v));
                            break;
                        } else {
                            stack.push(p);
                        }
                    }
                    while let Some(p) = stack.pop() {
                        new_seq.push(p);
                    }
                    new_seq.push(Item::Value(0));

                    let next = seq_iter.next().unwrap();
                    if let Item::Value(n) = next {
                        to_add = Some(n);
                    } else {
                        unreachable!()
                    }
                    seq_iter.next(); // skip over closing bracket.
                } else {
                    new_seq.push(item);
                }
            }
        }
    }
    new_seq.extend(seq_iter);
    (new_seq, changed)
}

fn reduce(seq: &[Item]) -> Vec<Item> {
    let (r1, c1) = explode(seq);
    if c1 {
        reduce(&r1)
    } else {
        let (r2, c2) = split(seq);
        if c2 {
            reduce(&r2)
        } else {
            seq.to_vec()
        }
    }
}

fn magnitude(seq: &[Item]) -> i64 {
    if seq.len() == 1 {
        if let Item::Value(n) = seq[0] {
            return n as i64;
        }
    }
    let mut count = 0;
    let mut idx = None;
    for (i, &item) in seq.iter().enumerate().skip(1) {
        match item {
            Item::Open => count += 1,
            Item::Close => count -= 1,
            Item::Value(_) => (),
        }
        if count == 0 {
            idx = Some(i + 1);
            break;
        }
    }
    let idx = idx.unwrap();
    3 * magnitude(&seq[1..idx]) + 2 * magnitude(&seq[idx..seq.len() - 1])
}

fn main() {
    let input = parse_input();

    let mut best = 0;

    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j {
                continue;
            }
            let mut new_seq = vec![Item::Open];
            new_seq.extend_from_slice(&input[i]);
            new_seq.extend_from_slice(&input[j]);
            new_seq.push(Item::Close);

            best = std::cmp::max(best, magnitude(&reduce(&new_seq)));
        }
    }

    println!("{}", best);
}
