use std::{collections::HashSet, io::Read};

fn parse_input() -> Vec<Vec<u8>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    input
        .trim()
        .split('\n')
        .map(|row| row.bytes().map(|x| x - 48).collect())
        .collect()
}

fn main() {
    let grid = parse_input();

    let mut visited = HashSet::new();

    let mut sizes = vec![];

    for (y, row) in grid.iter().enumerate() {
        for (x, &sq) in row.iter().enumerate() {
            if sq != 9 && !visited.contains(&(x, y)) {
                sizes.push(search(&grid, &mut visited, (x, y)));
            }
        }
    }

    sizes.sort_unstable();
    sizes.reverse();

    println!(
        "{}",
        sizes[..3].iter().copied().reduce(|a, b| a * b).unwrap()
    );
}

fn search(grid: &[Vec<u8>], visited: &mut HashSet<(usize, usize)>, (x, y): (usize, usize)) -> i32 {
    visited.insert((x, y));
    1 + [(0, -1), (-1, 0), (1, 0), (0, 1)]
        .iter()
        .filter_map(|&(dx, dy)| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0
                && nx < grid[0].len() as isize
                && ny >= 0
                && ny < grid.len() as isize
                && !visited.contains(&(nx as usize, ny as usize))
                && grid[ny as usize][nx as usize] != 9
            {
                Some(search(grid, visited, (nx as usize, ny as usize)))
            } else {
                None
            }
        })
        .sum::<i32>()
}
