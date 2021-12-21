use std::io::Read;

use itertools::Itertools;

fn parse_input() -> Image {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let parts = input.trim().split_once("\n\n").unwrap();

    Image::new(
        parts
            .0
            .replace("\n", "")
            .chars()
            .map(|c| c == '#')
            .collect(),
        parts
            .1
            .split('\n')
            .map(|row| row.chars().map(|c| c == '#').collect())
            .collect(),
    )
}

struct Image {
    enhancement: Vec<bool>,
    grid: Vec<Vec<bool>>,
    inf_pixel: bool,
}

impl Image {
    fn new(enhancement: Vec<bool>, grid: Vec<Vec<bool>>) -> Self {
        Self {
            enhancement,
            grid,
            inf_pixel: false,
        }
    }

    fn enhance(&mut self) {
        let mut new_grid = vec![vec![false; self.width() + 2]; self.height() + 2];
        for x in -1..self.width() as isize + 1 {
            for y in -1..self.height() as isize + 1 {
                new_grid[(y + 1) as usize][(x + 1) as usize] = self.read(x, y);
            }
        }
        self.grid = new_grid;
        self.inf_pixel = if self.inf_pixel {
            self.enhancement[511]
        } else {
            self.enhancement[0]
        }
    }

    fn read(&self, x: isize, y: isize) -> bool {
        let mut value = 0;
        for sq in (y - 1..=y + 1)
            .flat_map(|ny| (x - 1..=x + 1).map(move |nx| (nx, ny)))
            .map(|(nx, ny)| self.get(nx, ny))
        {
            value <<= 1;
            if sq {
                value += 1;
            }
        }
        self.enhancement[value]
    }

    fn count(&self, value: bool) -> Option<usize> {
        if self.inf_pixel == value {
            return None;
        }
        Some(self.grid.iter().flatten().filter(|&&v| v == value).count())
    }

    fn get(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 || x >= self.grid[0].len() as isize || y >= self.grid.len() as isize {
            return self.inf_pixel;
        }
        self.grid[y as usize][x as usize]
    }

    #[allow(dead_code)]
    fn grid_string(&self) -> String {
        self.grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&sq| if sq { '#' } else { '.' })
                    .collect::<String>()
            })
            .join("\n")
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }
}

fn main() {
    let mut image = parse_input();

    for _ in 0..2 {
        image.enhance();
    }
    println!("{}", image.count(true).unwrap());
}
