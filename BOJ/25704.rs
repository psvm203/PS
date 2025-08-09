use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let [n, p] = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect::<Vec<i32>>()
        .try_into()
        .unwrap();

    let mut min_cost = p;

    if n >= 5 {
        min_cost = 0.max(p - 500);
    }

    if n >= 10 {
        min_cost = min_cost.min(p * 9 / 10);
    }

    if n >= 15 {
        min_cost = min_cost.min(0.max(p - 2000));
    }

    if n >= 20 {
        min_cost = min_cost.min(p * 3 / 4);
    }

    println!("{min_cost}");
}
