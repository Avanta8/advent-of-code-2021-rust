use std::{collections::HashSet, io::Read};

fn parse_input() -> Vec<Vec<u8>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    input
        .trim()
        .split('\n')
        .map(|line| line.bytes().map(|x| x - b'0').collect())
        .collect()
}

struct Neighbour {
    width: usize,
    height: usize,
}

impl Neighbour {
    fn get(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .into_iter()
        .filter_map(|(dx, dy)| {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                Some((nx as usize, ny as usize))
            } else {
                None
            }
        })
        .collect()
    }
}

fn main() {
    let mut grid = parse_input();

    let n = Neighbour {
        width: grid[0].len(),
        height: grid.len(),
    };

    let mut idx = 0;
    loop {
        idx += 1;
        let mut visited = HashSet::new();
        let mut bag = vec![];

        for (y, row) in grid.iter_mut().enumerate() {
            for (x, sq) in row.iter_mut().enumerate() {
                *sq += 1;
                if *sq == 10 {
                    visited.insert((x, y));
                    bag.push((x, y));
                }
            }
        }

        while let Some((cx, cy)) = bag.pop() {
            for (nx, ny) in n.get((cx, cy)) {
                grid[ny][nx] += 1;
                if grid[ny][nx] == 10 && !visited.contains(&(nx, ny)) {
                    bag.push((nx, ny));
                    visited.insert((nx, ny));
                }
            }
        }

        let mut count = 0;
        for row in grid.iter_mut() {
            for sq in row.iter_mut() {
                if *sq >= 10 {
                    count += 1;
                    *sq = 0;
                }
            }
        }
        if count == n.width * n.height {
            break;
        }
    }

    println!("{}", idx);
}
