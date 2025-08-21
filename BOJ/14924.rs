use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let [s, t, d] = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let time = d / s / 2;
    let output = t * time;

    println!("{output}");
}
