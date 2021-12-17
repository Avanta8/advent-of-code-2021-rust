use std::io::Read;

use regex::Regex;

type Target = ((i64, i64), (i64, i64));

fn parse_input() -> Target {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let re = Regex::new(r"-?\d+").unwrap();

    let mut captures = re.find_iter(&input);
    let mut next = || captures.next().unwrap().as_str().parse::<_>().unwrap();
    ((next(), next()), (next(), next()))
}

fn simulate(target: Target, mut dx: i64, mut dy: i64) -> bool {
    let mut x = 0;
    let mut y = 0;
    loop {
        x += dx;
        y += dy;
        dx = std::cmp::max(0, dx - 1);
        dy -= 1;
        if target.0 .0 <= x && x <= target.0 .1 && target.1 .0 <= y && y <= target.1 .1 {
            break true;
        } else if x > target.0 .1 || y < target.1 .0 {
            break false;
        }
    }
}

fn main() {
    let target = parse_input();

    let mut count = 0;
    for dx in 0..=target.0 .1 {
        for dy in target.1 .0..-target.1 .0 {
            if simulate(target, dx, dy) {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
