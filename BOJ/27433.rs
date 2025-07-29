use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.trim().parse().unwrap();

    let factorial = (1..=n).fold(1, |acc, x| acc * x);

    println!("{factorial}");
}
