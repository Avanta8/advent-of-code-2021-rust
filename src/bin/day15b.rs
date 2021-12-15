use std::{cmp::Reverse, collections::BinaryHeap, io::Read};

use rustc_hash::FxHashMap;

fn parse_input() -> Vec<Vec<i64>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    input
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|sq| sq.to_string().parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn create_grid(orig: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let mut grid = vec![];
    for i in 0..5 {
        for orig_row in orig.iter() {
            let mut row = vec![];
            for j in 0..5 {
                row.extend(orig_row.iter().map(|&sq| (sq + i + j - 1) % 9 + 1));
            }
            grid.push(row);
        }
    }
    grid
}

fn main() {
    let grid = create_grid(&parse_input());

    let width = grid[0].len();
    let height = grid.len();

    let end = (width - 1, height - 1);

    let mut bag = BinaryHeap::new();
    let mut seen = FxHashMap::default();
    bag.push((Reverse(0), (0, 0)));
    seen.insert((0, 0), 0);

    while let Some((Reverse(dist), pos)) = bag.pop() {
        if pos == end {
            println!("{}", dist);
            break;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nx, ny) = (pos.0 as isize + dx, pos.1 as isize + dy);
            if nx < 0 || nx >= width as isize || ny < 0 || ny >= height as isize {
                continue;
            }
            let npos = (nx as usize, ny as usize);
            let new_dist = dist + grid[npos.1][npos.0];
            if seen.get(&npos).copied().unwrap_or(i64::MAX) > new_dist {
                seen.insert(npos, new_dist);
                bag.push((Reverse(new_dist), npos));
            }
        }
    }
}
