use std::cmp::{max, min};
use std::io::Read;

use regex::Regex;

fn parse_input() -> Vec<(bool, Cuboid)> {
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
                Cuboid::new((next(), next()), (next(), next()), (next(), next())),
            )
        })
        .collect()
}

#[derive(Debug)]
struct Cuboid {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl Cuboid {
    fn new(x: (i64, i64), y: (i64, i64), z: (i64, i64)) -> Self {
        Self { x, y, z }
    }

    fn intersects(&self, other: &Cuboid) -> bool {
        intersects_1d(self.x, other.x)
            && intersects_1d(self.y, other.y)
            && intersects_1d(self.z, other.z)
    }

    fn volume(&self) -> i64 {
        (self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)
    }
}

#[derive(Debug, Default)]
struct Grid {
    cuboids: Vec<(Cuboid, bool)>,
}

impl Grid {
    fn add_cuboid(&mut self, new: Cuboid, state: bool) {
        let mut new_cuboids = Vec::with_capacity(self.cuboids.capacity());
        for (old, old_state) in std::mem::take(&mut self.cuboids) {
            if !new.intersects(&old) {
                new_cuboids.push((old, old_state));
            } else {
                new_cuboids.extend(separate(&old, &new).map(|c| (c, old_state)));
            }
        }
        new_cuboids.push((new, state));

        self.cuboids = new_cuboids;
    }

    fn count_on(&self) -> i64 {
        self.cuboids
            .iter()
            .filter_map(|(cuboid, state)| state.then(|| cuboid.volume()))
            .sum()
    }
}

fn intersects_1d(a: (i64, i64), b: (i64, i64)) -> bool {
    max(a.0, b.0) <= min(a.1, b.1)
}

fn new_sides(old: (i64, i64), new: (i64, i64)) -> impl Iterator<Item = (i64, i64)> {
    /*
    eg. given:
     ---------------
    a0             a1
          ------
          b0  b1

    returns:
     ----|------|---
    */
    [
        (old.0, new.0 - 1),
        (max(old.0, new.0), min(old.1, new.1)),
        (new.1 + 1, old.1),
    ]
    .into_iter()
    .filter(|&(a, b)| b >= a)
}

/// Returns an iterator of cuboids that combine to exactly the area
/// inside `old` but outside `new`.
fn separate(old: &Cuboid, new: &Cuboid) -> impl Iterator<Item = Cuboid> {
    let mut seperated = vec![];
    for bx in new_sides(old.x, new.x) {
        for by in new_sides(old.y, new.y) {
            for bz in new_sides(old.z, new.z) {
                let cub = Cuboid::new(bx, by, bz);
                // If `new` intersects `cub`, then `cub` must be fully inside `new`, and
                // we don't want to incluce any volume that comes from `new`.
                if !new.intersects(&cub) {
                    seperated.push(cub);
                }
            }
        }
    }
    seperated.into_iter()
}

fn main() {
    let input = parse_input();

    let mut grid = Grid::default();
    for (state, cuboid) in input {
        grid.add_cuboid(cuboid, state);
    }

    println!("{}", grid.count_on());
}
