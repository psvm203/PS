use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let nums: Vec<usize> = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    let sum: usize = nums[0..4].iter().sum();
    let supplement = (nums[4] * 4) as i32 - sum as i32;
    let supplement = 0.max(supplement);

    println!("{supplement}");
}
