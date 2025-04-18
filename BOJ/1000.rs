use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let sum: usize = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .sum();

    print!("{sum}");
}
