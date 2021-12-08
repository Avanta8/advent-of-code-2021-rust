use std::{cmp::min, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut crabs = input
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    crabs.sort_unstable();

    let len = crabs.len() as i32;

    let mut left = 0;
    let mut right = crabs.iter().sum::<i32>();
    let mut pos = 0;

    let mut best = right;

    for (count, next) in (0..).zip(crabs) {
        for _ in pos..next {
            right -= len - count;
            left += count;
            best = min(best, right + left);
        }
        pos = next;
    }

    println!("{}", best);
}
