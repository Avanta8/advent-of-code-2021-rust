use itertools::Itertools;
use lazy_static::lazy_static;
use std::{
    collections::{BTreeSet, HashMap},
    io::Read,
};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let counts = input.trim().split('\n').map(|row| {
        solve_one(
            &row.trim()
                .replace('|', "")
                .split_ascii_whitespace()
                .map(|p| p.bytes().map(|b| (b - 97) as usize).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
    });

    println!("{}", counts.sum::<i32>());
}

lazy_static! {
    static ref PERMUTATIONS: Vec<Vec<u8>> = (0..7).permutations(7).collect::<Vec<_>>();
    static ref DISPLAYS: HashMap<BTreeSet<u8>, i32> = [
        ([0, 1, 2, 4, 5, 6].into(), 0),
        ([2, 5].into(), 1),
        ([0, 2, 3, 4, 6].into(), 2),
        ([0, 2, 3, 5, 6].into(), 3),
        ([1, 2, 3, 5].into(), 4),
        ([0, 1, 3, 5, 6].into(), 5),
        ([0, 1, 3, 4, 5, 6].into(), 6),
        ([0, 2, 5].into(), 7),
        ([0, 1, 2, 3, 4, 5, 6].into(), 8),
        ([0, 1, 2, 3, 5, 6].into(), 9),
    ]
    .into();
}

fn solve_one(signals: &[Vec<usize>]) -> i32 {
    for perm in PERMUTATIONS.iter() {
        let mut positions = signals
            .iter()
            .map(|seg| seg.iter().map(|&x| perm[x]).collect::<BTreeSet<_>>());

        if positions
            .by_ref()
            .take(10)
            .all(|seg| DISPLAYS.contains_key(&seg))
        {
            return positions
                .map(|seg| DISPLAYS.get(&seg).unwrap().to_string())
                .join("")
                .parse()
                .unwrap();
        }
    }
    unreachable!()
}
