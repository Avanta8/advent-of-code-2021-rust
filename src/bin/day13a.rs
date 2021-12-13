use std::io::Read;

use itertools::Itertools;

#[allow(clippy::type_complexity)]
fn parse_input() -> (Vec<(usize, usize)>, Vec<(bool, usize)>) {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (points, folds) = input.trim().split_once("\n\n").unwrap();

    (
        points
            .split('\n')
            .map(|line| {
                let coord = line.split_once(',').unwrap();
                (coord.0.parse().unwrap(), coord.1.parse().unwrap())
            })
            .collect(),
        folds
            .split('\n')
            .map(|line| {
                (
                    line.contains('x'),
                    line.split_once('=').unwrap().1.parse().unwrap(),
                )
            })
            .collect(),
    )
}

fn main() {
    let (points, folds) = parse_input();

    let (fold, line) = folds[0];
    let count = points
        .into_iter()
        .map(|(x, y)| {
            if fold && x < line || !fold && y < line {
                (x, y)
            } else if fold {
                (2 * line - x, y)
            } else {
                (x, 2 * line - y)
            }
        })
        .unique()
        .count();

    println!("{}", count);
}
