use std::{collections::HashSet, io::Read};

#[allow(clippy::type_complexity)]
fn parse_input() -> (HashSet<(usize, usize)>, Vec<(bool, usize)>) {
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
    let (mut points, folds) = parse_input();

    for (fold, line) in folds {
        points = points
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
            .collect();
    }

    display(&points);
}

fn display(points: &HashSet<(usize, usize)>) {
    let width = points.iter().map(|&p| p.0).max().unwrap() + 1;
    let height = points.iter().map(|&p| p.1).max().unwrap() + 1;
    let mut grid = vec![vec![' '; width]; height];

    for &(x, y) in points {
        grid[y][x] = '#';
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}
