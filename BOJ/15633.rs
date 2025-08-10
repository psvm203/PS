use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: i32 = stdin.trim().parse().unwrap();

    let sum: i32 = (1..=n).filter(|x| n % x == 0).sum();
    let output = sum * 5 - 24;

    println!("{output}");
}
