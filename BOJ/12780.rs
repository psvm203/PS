use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let [h, n] = stdin
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let count = h.matches(n).count();

    println!("{count}");
}
