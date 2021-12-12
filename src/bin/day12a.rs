use std::{collections::HashMap, io::Read};

fn parse_input() -> (HashMap<String, Vec<String>>, HashMap<String, u32>) {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut bit: u32 = 1;
    let mut flags = HashMap::new();

    let mut graph = HashMap::new();
    for row in input.trim().split('\n') {
        let line = row.split_once('-').unwrap();
        let (a, b) = (line.0.to_string(), line.1.to_string());

        for node in [a.clone(), b.clone()] {
            if !flags.contains_key(&node) && node.chars().all(char::is_lowercase) {
                flags.insert(node, bit);
                bit <<= 1;
            }
        }

        graph
            .entry(a.clone())
            .or_insert_with(Vec::new)
            .push(b.clone());
        graph.entry(b).or_insert_with(Vec::new).push(a);
    }

    (graph, flags)
}

fn main() {
    let (graph, flags) = parse_input();

    let mut count = 0;
    let mut bag = vec![("start", *flags.get("start").unwrap())];
    while let Some((pos, seen)) = bag.pop() {
        if pos == "end" {
            count += 1;
            continue;
        }

        for neighbour in graph.get(pos).unwrap() {
            if !neighbour.chars().all(char::is_lowercase)
                || seen | flags.get(neighbour).unwrap() != seen
            {
                bag.push((neighbour, seen | flags.get(neighbour).unwrap_or(&0)));
            }
        }
    }

    println!("{}", count);
}
