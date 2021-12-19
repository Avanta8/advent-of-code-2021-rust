use rustc_hash::FxHashSet;
use std::{hash::Hash, io::Read};

/// Point uses left handed 3d cartesian coordinate system.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point(i32, i32, i32);

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Point {
    /// Returns a rotation 90 degrees clockwise around the x-axis.
    fn rotate_x(&self) -> Self {
        Self(self.0, -self.2, self.1)
    }

    /// Returns a rotation 90 degrees clockwise around the y-axis.
    fn rotate_y(&self) -> Self {
        Self(self.2, self.1, -self.0)
    }

    /// Returns a rotation 90 degrees clockwise around the z-axis.
    fn rotate_z(&self) -> Self {
        Self(-self.1, self.0, self.2)
    }

    fn manhattan(&self) -> i32 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

fn parse_input() -> Vec<Vec<Point>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    input
        .trim()
        .split("\n\n")
        .map(|scanner| {
            scanner
                .split('\n')
                .skip(1)
                .map(|row| {
                    let mut it = row.split(',');
                    let mut next = || it.next().unwrap().parse().unwrap();
                    Point(next(), next(), next())
                })
                .collect()
        })
        .collect()
}

fn get_rotations(points: &[Point]) -> Vec<Vec<Point>> {
    let mut new_points = vec![points.to_owned()];
    for _ in 0..3 {
        new_points.push(
            new_points
                .last()
                .unwrap()
                .iter()
                .map(|&point| point.rotate_z())
                .collect(),
        );
    }
    {
        let mut x_rots = [vec![], vec![]];
        for &point in points {
            let np = point.rotate_x();
            x_rots[0].push(np);
            x_rots[1].push(np.rotate_x().rotate_x());
        }
        new_points.extend(x_rots);
    }

    let mut rotations = vec![];

    for points in new_points {
        rotations.push(points);
        for _ in 0..3 {
            rotations.push(
                rotations
                    .last()
                    .unwrap()
                    .iter()
                    .map(|&point| point.rotate_y())
                    .collect(),
            )
        }
    }
    rotations
}

fn is_intersect(a: &[Point], b: &[Point]) -> Option<(Vec<Point>, Point)> {
    let sa = a.iter().copied().collect::<FxHashSet<_>>();
    for rot_b in get_rotations(b) {
        for &bp in rot_b.iter().skip(11) {
            // If we have a match, then 12 other points will also match. If we know
            // that len - 11 points don't match, then we know that this rotation doesn't match.
            for &ap in a {
                let dp = ap - bp;

                let mut count = 0;
                let new_b = rot_b.iter().map(|&p| p + dp);
                for (idx, point) in new_b.clone().enumerate() {
                    if sa.contains(&point) {
                        count += 1;
                    }
                    if count >= 12 {
                        return Some((new_b.collect(), dp));
                    } else if b.len() - idx - 1 < 12 - count {
                        break;
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let scanners = parse_input();

    let length = scanners.len();
    let mut dists = vec![None; length];
    dists[0] = Some(Point(0, 0, 0));

    let mut bag = vec![scanners.get(0).unwrap().to_owned()];
    while let Some(current) = bag.pop() {
        for idx in 0..length {
            if dists[idx].is_some() {
                continue;
            }
            if let Some((np, dp)) = is_intersect(&current, &scanners[idx]) {
                dists[idx] = Some(dp);
                bag.push(np);
            }
        }
    }

    let mut best = 0;
    for a in dists.iter().filter_map(|&p| p) {
        for b in dists.iter().filter_map(|&p| p) {
            best = std::cmp::max(best, (a - b).manhattan());
        }
    }
    println!("{}", best);
}
