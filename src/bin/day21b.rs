use cached::proc_macro::cached;
use std::io::Read;

fn parse_input() -> (i64, i64) {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let starts = input.trim().split_once('\n').unwrap();
    (
        starts.0.rsplit_once(' ').unwrap().1.parse().unwrap(),
        starts.1.rsplit_once(' ').unwrap().1.parse().unwrap(),
    )
}

#[cached]
fn find(p1: i64, p2: i64, t1: i64, t2: i64) -> (i64, i64) {
    if t2 <= 0 {
        return (0, 1);
    }
    let mut total = (0, 0);
    for a in 1..=3 {
        for b in 1..=3 {
            for c in 1..=3 {
                let np = (p1 + a + b + c - 1) % 10 + 1;
                let res = find(p2, np, t2, t1 - np);
                total.0 += res.1;
                total.1 += res.0;
            }
        }
    }
    total
}

fn main() {
    let (p1, p2) = parse_input();

    let f = find(p1, p2, 21, 21);
    println!("{:?}", f);
}
