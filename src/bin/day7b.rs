use std::{cmp::min, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let crabs = input
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut best = i32::MAX;

    for i in 0..=*crabs.iter().max().unwrap() {
        let mut total = 0;
        for &crab in crabs.iter() {
            let dist = (i - crab).abs();
            total += dist * (dist + 1) / 2;
        }
        best = min(best, total);
    }

    println!("{}", best);
}
