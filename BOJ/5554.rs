use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let sum: usize = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .sum();

    println!("{}\n{}", sum / 60, sum % 60);
}
