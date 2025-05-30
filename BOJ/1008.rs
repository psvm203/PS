use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let numbers: Vec<f64> = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    let output = numbers[0] / numbers[1];

    print!("{output}");
}
