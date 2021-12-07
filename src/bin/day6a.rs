use std::io::Read;

fn parse_input() -> Vec<usize> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let input = parse_input();

    let mut fish = vec![0; 9];
    for f in input {
        fish[f] += 1;
    }

    for _ in 0..80 {
        fish[7] += fish[0];
        fish.rotate_left(1);
    }

    let ans = fish.iter().sum::<u64>();
    println!("{}", ans);
}
