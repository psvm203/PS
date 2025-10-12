use std::collections::*;
use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let lines = stdin.lines().skip(1);

    let mut counts = HashMap::new();

    lines.for_each(|line| {
        let [fruit, count] = line
            .split_ascii_whitespace()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let count: usize = count.parse().unwrap();

        counts
            .entry(fruit)
            .and_modify(|x| *x += count)
            .or_insert(count);
    });

    let output = if counts.values().any(|count| *count == 5) {
        "YES"
    } else {
        "NO"
    };

    println!("{output}");
}
