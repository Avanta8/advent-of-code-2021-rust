use std::cmp::{max, min};
use std::{collections::HashSet, io::Read};

use regex::Regex;

#[allow(clippy::type_complexity)]
fn parse_input() -> Vec<(bool, (i32, i32), (i32, i32), (i32, i32))> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let re = Regex::new(r"-?\d+").unwrap();
    input
        .trim()
        .split('\n')
        .map(|row| {
            let mut captures = re.find_iter(row);
            let mut next = || captures.next().unwrap().as_str().parse::<_>().unwrap();
            (
                row.starts_with("on"),
                (next(), next()),
                (next(), next()),
                (next(), next()),
            )
        })
        .collect()
}

fn main() {
    let input = parse_input();

    let mut grid = HashSet::new();
    for (command, cx, cy, cz) in input {
        for px in max(cx.0, -50)..=min(cx.1, 50) {
            for py in max(cy.0, -50)..=min(cy.1, 50) {
                for pz in max(cz.0, -50)..=min(cz.1, 50) {
                    if command {
                        grid.insert((px, py, pz));
                    } else {
                        grid.remove(&(px, py, pz));
                    }
                }
            }
        }
    }
    println!("{}", grid.len());
}
