use std::io::{self, Read};

fn parse_input() -> Vec<(String, i32)> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    input
        .trim()
        .split('\n')
        .map(|line| {
            let mut s = line.split_ascii_whitespace();
            (
                s.next().unwrap().to_string(),
                s.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect()
}

fn main() {
    let input = parse_input();

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for (d, s) in input {
        match d.as_str() {
            "forward" => {
                x += s;
                y += aim * s;
            }
            "backward" => x -= s,
            "up" => aim -= s,
            "down" => aim += s,
            _ => unreachable!(),
        }
    }
    println!("{}", x * y);
}
