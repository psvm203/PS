use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let nums = stdin
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse())
        .collect::<Vec<usize>>();

    let output = nums
        .chunks(2)
        .map(|chunk| {
            let [n, m] = chunk.try_into().unwrap();
            format!("{} {}", m * 2 - n, n - m)
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
