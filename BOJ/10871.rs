use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let x: usize = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();

    let numbers = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .flat_map(|number| number.parse::<usize>());

    let answer = numbers
        .filter(|&number| number < x)
        .map(|number| number.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    print!("{answer}");
}
