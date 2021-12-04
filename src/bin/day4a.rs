use std::{collections::HashMap, io::Read};

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<u8>>,
    map: HashMap<u8, (usize, usize)>,
    marked: Vec<Vec<bool>>,
}

impl Grid {
    fn new(grid: Vec<Vec<u8>>) -> Self {
        // let v = (0..5).flat_map(|y| (0..5).map(move |x| (grid[y][x], (x, y))));
        let v = (0..5)
            .flat_map(|y| (0..5).map(move |x| (x, y)))
            .map(|(x, y)| (grid[y][x], (x, y)));
        let map = HashMap::from_iter(v);
        Self {
            grid,
            map,
            marked: vec![vec![false; 5]; 5],
        }
    }

    fn mark(&mut self, sq: u8) -> bool {
        if let Some(&(x, y)) = self.map.get(&sq) {
            self.marked[y][x] = true;
            self.solved(x, y)
        } else {
            false
        }
    }

    fn solved(&self, x: usize, y: usize) -> bool {
        (0..5).all(|nx| self.marked[y][nx]) || (0..5).all(|ny| self.marked[ny][x])
    }

    fn value(&self) -> u32 {
        (0..5)
            .flat_map(|y| {
                (0..5).map(move |x| {
                    if !self.marked[y][x] {
                        self.grid[y][x] as u32
                    } else {
                        0
                    }
                })
            })
            .sum()
    }
}

fn parse_input() -> (Vec<u8>, Vec<Grid>) {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut grids = vec![];

    let mut lines = input.lines().peekable();
    let clues = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect();
    lines.next();

    while lines.peek().is_some() {
        let grid = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        grids.push(Grid::new(grid));
    }
    (clues, grids)
}

fn main() {
    let (squares, mut grids) = parse_input();

    for sq in squares {
        for grid in grids.iter_mut() {
            if grid.mark(sq) {
                println!("{}", grid.value() * sq as u32);
                return;
            }
        }
    }
}
