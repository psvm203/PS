use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let sum: i32 = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<i32>())
        .sum();

    print!("{sum}");
}
