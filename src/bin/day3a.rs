use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let v = input
        .trim()
        .split('\n')
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let c = (0..v[0].len())
        .map(|i| {
            if v.iter().filter(|row| row[i] == '1').count() > v.len() / 2 {
                '1'
            } else {
                '0'
            }
        })
        .collect::<String>();
    let g = u32::from_str_radix(&c, 2).unwrap();
    let e = !(g << (32 - v[0].len())) >> (32 - v[0].len());

    println!("{}", e * g);
}
