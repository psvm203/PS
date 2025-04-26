use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let mut opinions: Vec<usize> = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse())
        .collect();

    opinions.sort();

    let exclude_count = (opinions.len() as f64 * 0.15).round() as usize;

    let trimmed_opinions = &opinions[exclude_count..opinions.len() - exclude_count];

    let level = (trimmed_opinions.into_iter().sum::<usize>() as f64 / trimmed_opinions.len() as f64)
        .round() as usize;

    print!("{level}");
}
