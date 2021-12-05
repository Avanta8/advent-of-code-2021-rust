use regex::Regex;
use std::cmp::max;
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
    for ((mut x, mut y), (c, d)) in input {
        let dx = (c - x).signum();
        let dy = (d - y).signum();
        for _ in 0..=max((x - c).abs(), (y - d).abs()) {
            *grid.entry((x, y)).or_insert(0) += 1;
            x += dx;
            y += dy;
        }
    }

    let ans = grid.iter().filter(|(_, &c)| c >= 2).count();
    println!("{}", ans);
}
