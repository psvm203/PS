use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let calculate_sum = |line: &str| {
        let sum: usize = line
            .split_ascii_whitespace()
            .flat_map(|x| x.parse::<usize>())
            .sum();

        sum
    };

    let sums = input
        .lines()
        .skip(1)
        .map(|line| calculate_sum(line).to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{sums}");
}
