use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let nums: Vec<_> = stdin
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| u128::from_str_radix(x, 2))
        .collect();

    let output = nums
        .chunks(2)
        .map(|chunk| format!("{:b}", chunk[0] + chunk[1]))
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
