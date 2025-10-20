use std::io::*;

fn solve(x: usize) -> usize {
    let rt = (x as f64).sqrt() as usize;

    if rt * rt == x {
        1
    } else {
        0
    }
}

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let nums = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .skip(1);

    let output = nums
        .map(|num| solve(num).to_string())
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
