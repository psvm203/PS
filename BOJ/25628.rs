use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let nums: Vec<usize> = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    let count = (nums[0] / 2).min(nums[1]);

    println!("{count}");
}
