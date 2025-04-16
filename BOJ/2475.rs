use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let numbers = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<i32>());

    let checksum = numbers.map(|x| x * x).sum::<i32>() % 10;

    print!("{checksum}");
}
