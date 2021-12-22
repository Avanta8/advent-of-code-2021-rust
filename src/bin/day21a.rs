use std::io::Read;

fn parse_input() -> (i32, i32) {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let starts = input.trim().split_once('\n').unwrap();
    (
        starts.0.rsplit_once(' ').unwrap().1.parse().unwrap(),
        starts.1.rsplit_once(' ').unwrap().1.parse().unwrap(),
    )
}

fn main() {
    let (p1, p2) = parse_input();

    let mut positions = [p1, p2];
    let mut scores = [0, 0];
    let mut current = 0;
    let mut count = 1..;

    for _ in 0.. {
        for _ in 0..3 {
            positions[current] = (positions[current] + count.next().unwrap() - 1) % 10 + 1;
        }
        scores[current] += positions[current];
        if scores[current] >= 1000 {
            println!("{}", scores[current ^ 1] * (count.next().unwrap() - 1));
            break;
        }
        current ^= 1;
    }
}
