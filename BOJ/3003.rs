use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let counts: Vec<usize> = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    let correct_counts = vec![1, 1, 2, 2, 2, 8];

    let output = counts
        .iter()
        .zip(correct_counts.iter())
        .map(|(&a, &b)| (b - a as i32).to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{output}");
}
