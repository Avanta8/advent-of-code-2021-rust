use std::{collections::HashMap, io::Read};

use itertools::Itertools;

fn parse_input() -> (Vec<char>, HashMap<char, HashMap<char, char>>) {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let (seq, tmpl) = input.trim().split_once("\n\n").unwrap();

    let mut rules = HashMap::new();
    for mut line in tmpl.split('\n').map(|row| row.chars()) {
        rules
            .entry(line.next().unwrap())
            .or_insert_with(HashMap::new)
            .insert(line.next().unwrap(), line.nth(4).unwrap());
    }

    (seq.chars().collect(), rules)
}

fn main() {
    let (mut seq, rules) = parse_input();

    for _ in 0..10 {
        let mut new_seq = Vec::with_capacity(seq.capacity());
        new_seq.push(seq[0]);

        for (&a, &b) in seq.iter().zip(seq.iter().skip(1)) {
            if let Some(p) = rules.get(&a) {
                if let Some(q) = p.get(&b) {
                    new_seq.push(*q);
                }
            }
            new_seq.push(b);
        }

        seq = new_seq;
    }

    let (n, m) = seq
        .iter()
        .counts()
        .values()
        .copied()
        .minmax()
        .into_option()
        .unwrap();
    println!("{}", m - n);
}
