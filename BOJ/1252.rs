use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let to_num = |s: &str| u128::from_str_radix(s, 2).unwrap();

    let output = stdin.split_ascii_whitespace().map(to_num).sum::<u128>();

    println!("{:b}", output);
}
