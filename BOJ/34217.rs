use std::cmp::Ordering::*;
use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let [a, b, c, d] = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    let output = match (a + c).cmp(&(b + d)) {
        Less => "Hanyang Univ.",
        Greater => "Yongdap",
        Equal => "Either",
    };

    println!("{output}");
}
