use std::io::*;

const MODULUS: usize = 10007;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut memo = [0usize; 1001];
    memo[1] = 1;
    memo[2] = 3;

    (3..=n).for_each(|i| memo[i] = (memo[i - 1] + memo[i - 2] * 2) % MODULUS);

    let output = memo[n];

    print!("{output}");
}
