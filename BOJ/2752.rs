use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let mut nums = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect::<Vec<usize>>();

    nums.sort();

    let output = nums
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
