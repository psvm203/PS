use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let calculate_score = |line: &str| {
        let score: usize = line
            .split('X')
            .map(|s| s.chars().count())
            .map(|x| x * (x + 1) / 2)
            .sum();

        score
    };

    let scores = input
        .lines()
        .skip(1)
        .map(|line| calculate_score(line).to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{scores}");
}
