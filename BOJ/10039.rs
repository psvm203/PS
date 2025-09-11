use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let scores = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>());

    let output = scores.map(|score| score.max(40)).sum::<usize>() / 5;

    println!("{output}");
}
