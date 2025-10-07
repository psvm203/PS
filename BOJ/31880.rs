use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let mut nums = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<u128>());

    let n = nums.next().unwrap() as usize;
    let m = nums.next().unwrap() as usize;
    let sum: u128 = (&mut nums).take(n).sum();
    let product: u128 = nums.take(m).filter(|x| *x != 0).product();

    let output = sum * product;

    println!("{output}");
}
