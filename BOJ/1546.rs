use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let numbers = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse::<usize>());

    let max = numbers.clone().max().unwrap() as f64;
    let average = numbers.clone().sum::<usize>() as f64 * 100.0 / max / numbers.count() as f64;

    print!("{average}");
}
