use std::io::Read;

use regex::Regex;
fn parse_input() -> ((i64, i64), (i64, i64)) {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let re = Regex::new(r"-?\d+").unwrap();

    let mut captures = re.find_iter(&input);
    let mut next = || captures.next().unwrap().as_str().parse::<_>().unwrap();
    ((next(), next()), (next(), next()))
}

fn main() {
    let ((_, _), (y1, _)) = parse_input();

    // We reach the start height with the same magnitude of y-velocity as we start with but
    // in the opposite direction. So the next step will have a velocity of -(start velocity + 1),
    // and we want to be at the lowest valid y coordinate after this step.

    let v = -y1 - 1;
    println!("{}", v * (v + 1) / 2);
}
