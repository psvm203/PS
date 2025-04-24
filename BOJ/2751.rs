use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut numbers: Vec<i32> = input.lines().skip(1).flat_map(|x| x.parse()).collect();

    numbers.sort();

    let output = numbers
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
