use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let sum: u128 = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<u128>())
        .sum();

    print!("{sum}");
}
