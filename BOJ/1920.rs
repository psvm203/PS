use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let set: std::collections::HashSet<&str> =
        lines.nth(1).unwrap().split_ascii_whitespace().collect();

    let output = lines
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| (set.contains(x) as usize).to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
