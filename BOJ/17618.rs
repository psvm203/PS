use std::io::*;

fn sum_digits(x: usize) -> usize {
    x.to_string()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .sum()
}

fn is_amazing(x: usize) -> bool {
    x % sum_digits(x) == 0
}

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.trim().parse().unwrap();

    let output = (1..=n).filter(|i| is_amazing(*i)).count();

    println!("{output}");
}
