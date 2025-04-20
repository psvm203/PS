use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let numbers: Vec<i32> = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    let (a, b) = (numbers[0], numbers[1]);

    let answer = [a + b, a - b, a * b, a / b, a % b]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{answer}");
}
