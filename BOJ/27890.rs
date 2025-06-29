use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let nums: Vec<usize> = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    let [mut x, n] = [nums[0], nums[1]];

    for _ in 0..n {
        if x % 2 == 0 {
            x = x / 2 ^ 6;
        } else {
            x = x * 2 ^ 6;
        }
    }

    println!("{x}");
}
