use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let mut read_split = || lines.next().unwrap().split_ascii_whitespace();

    let x: usize = read_split().nth(1).unwrap().parse().unwrap();
    let numbers = read_split().flat_map(|number| number.parse::<usize>());

    let output = numbers
        .filter(|&number| number < x)
        .map(|number| number.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    print!("{output}");
}
