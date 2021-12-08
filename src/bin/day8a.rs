use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let counts = input.trim().split('\n').map(|row| {
        row.split(' ')
            .skip(11)
            .filter(|s| [2, 3, 4, 7].contains(&s.len()))
            .count()
    });

    println!("{}", counts.sum::<usize>());
}
