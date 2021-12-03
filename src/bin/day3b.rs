use std::cmp::Ordering;
use std::io::{self, Read};

fn find(v: &[Vec<u8>], t: u8) -> u32 {
    let mut current = v.to_vec();
    for i in 0..v[0].len() {
        if current.len() == 1 {
            break;
        }
        let keep =
            match (current.iter().filter(|row| row[i] == 1).count() * 2).cmp(&(current.len())) {
                Ordering::Less => t ^ 1,
                _ => t,
            };
        current = current
            .into_iter()
            .filter(|b| b[i] == keep)
            .collect::<Vec<_>>();
    }

    u32::from_str_radix(
        &current[0]
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(""),
        2,
    )
    .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let v = input
        .trim()
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|x| x.to_string().parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{}", find(&v, 1) * find(&v, 0));
}
