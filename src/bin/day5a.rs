use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::io::Read;

fn parse_input() -> Vec<((i32, i32), (i32, i32))> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let re = Regex::new(r"\d+").unwrap();

    input
        .trim()
        .split('\n')
        .map(|line| {
            let mut captures = re.find_iter(line);
            let mut next = || captures.next().unwrap().as_str().parse::<_>().unwrap();
            ((next(), next()), (next(), next()))
        })
        .collect()
}

fn main() {
    let input = parse_input();

    let mut grid = HashMap::new();

    for ((a, b), (c, d)) in input {
        if a == c {
            for y in min(b, d)..=max(b, d) {
                *grid.entry((a, y)).or_insert(0) += 1;
            }
        } else if b == d {
            for x in min(a, c)..=max(a, c) {
                *grid.entry((x, b)).or_insert(0) += 1;
            }
        }
    }

    let ans = grid.iter().filter(|(_, &c)| c >= 2).count();
    println!("{}", ans);
}
