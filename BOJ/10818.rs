use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let numbers = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse::<i32>());

    let min: i32 = numbers.clone().min().unwrap();
    let max: i32 = numbers.max().unwrap();
    print!("{min} {max}");
}
