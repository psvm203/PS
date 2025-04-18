use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let numbers = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>());

    let checksum = numbers.map(|x| x * x).sum::<usize>() % 10;

    print!("{checksum}");
}
