use std::collections::HashMap;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let mut read_ints = || {
        lines
            .nth(1)
            .unwrap()
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
    };

    let cards = read_ints();
    let queries = read_ints();

    let mut counts = HashMap::new();

    cards.for_each(|x: i32| {
        *counts.entry(x).or_insert(0) += 1;
    });

    let output = queries
        .map(|x: i32| counts.get(&x).unwrap_or(&0).to_string())
        .collect::<Vec<_>>()
        .join(" ");

    print!("{output}");
}
