use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let [n, w, h, l] = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    let count = n.min((w / l) * (h / l));

    println!("{count}");
}
