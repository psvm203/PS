use std::collections::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines().skip(1);

    let recipe = lines.next().unwrap();
    let input: HashSet<_> = lines.next().unwrap().split_ascii_whitespace().collect();

    let miss = recipe
        .split_ascii_whitespace()
        .find(|&x| !input.contains(x))
        .unwrap();

    println!("{miss}");
}
