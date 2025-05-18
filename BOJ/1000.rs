use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let sum: usize = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .sum();

    print!("{sum}");
}
