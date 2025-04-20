use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let sum: usize = input
        .lines()
        .nth(1)
        .unwrap()
        .chars()
        .map(|ch| ch as usize - '0' as usize)
        .sum();

    print!("{sum}");
}
