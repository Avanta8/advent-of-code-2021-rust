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
    let (seq, rules) = parse_input();

    let mut results = HashMap::new();

    for (&first, seconds) in rules.iter() {
        for &second in seconds.keys() {
            let mut seg = vec![first, second];
            for _ in 0..20 {
                let mut new_seg = Vec::with_capacity(seg.capacity());
                new_seg.push(seg[0]);

                for (&a, &b) in seg.iter().zip(seg.iter().skip(1)) {
                    if let Some(p) = rules.get(&a) {
                        if let Some(q) = p.get(&b) {
                            new_seg.push(*q);
                        }
                    }
                    new_seg.push(b);
                }
                seg = new_seg;
            }
            results.insert((first, second), seg[1..].to_vec());
        }
    }

    let counts = results
        .iter()
        .map(|(&(first, second), seg)| ((first, second), seg.iter().copied().counts()))
        .collect::<HashMap<_, _>>();

    let mut inter_seq = vec![seq[0]];
    for (&a, &b) in seq.iter().zip(seq.iter().skip(1)) {
        inter_seq.extend(results[&(a, b)].iter());
    }

    let mut res_counts = HashMap::new();
    res_counts.insert(inter_seq[0], 1);
    for (&a, &b) in inter_seq.iter().zip(inter_seq.iter().skip(1)) {
        for (&c, &count) in counts[&(a, b)].iter() {
            *res_counts.entry(c).or_insert(0) += count;
        }
    }

    let (n, m) = res_counts.values().minmax().into_option().unwrap();
    println!("{}", m - n);
}
