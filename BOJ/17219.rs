use std::collections::HashMap;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let n: usize = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let passwords = lines.clone().take(n).map(|line| {
        let vec: Vec<_> = line.split_ascii_whitespace().collect();
        (vec[0], vec[1])
    });

    let queries = lines.skip(n);

    let map: HashMap<_, _> = passwords.collect();

    let output = queries
        .map(|query| map[query])
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
