use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines();

    let [_, x] = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    let buses: Vec<Vec<usize>> = lines
        .map(|line| {
            line.split_ascii_whitespace()
                .flat_map(|x| x.parse())
                .collect()
        })
        .collect();

    let output = buses
        .into_iter()
        .filter(|bus| bus[0] + bus[1] <= x)
        .map(|bus| bus[0] as i32)
        .fold(-1, |mx, s| mx.max(s));

    println!("{output}");
}
