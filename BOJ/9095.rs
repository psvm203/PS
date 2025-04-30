use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let test_cases = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse::<usize>());

    let mut memo = [0; 11];
    memo[1] = 1;
    memo[2] = 2;
    memo[3] = 4;

    (4..11).for_each(|i| {
        memo[i] = memo[i - 1] + memo[i - 2] + memo[i - 3];
    });

    let output = test_cases
        .map(|test_case| memo[test_case].to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
