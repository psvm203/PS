use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let [a, b] = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect::<Vec<i128>>()
        .try_into()
        .unwrap();

    let output = a * a - b * b;

    println!("{output}");
}
