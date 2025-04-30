use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let test_cases = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse::<usize>());

    let mut padovan_sequence = [1usize; 101];

    (4..101).for_each(|i| padovan_sequence[i] = padovan_sequence[i - 2] + padovan_sequence[i - 3]);

    let output = test_cases
        .map(|test_case| padovan_sequence[test_case].to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
