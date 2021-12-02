use std::error::Error;
use std::io;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let depth = input
        .trim()
        .split('\n')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let count = depth
        .iter()
        .zip(depth.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count();

    println!("{}", count);

    Ok(())
}
