use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let nums: Vec<usize> = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    let n = nums[0];
    let outlets = &nums[2..];
    let available: usize = outlets.iter().map(|outlet| (outlet + 1) / 2).sum();
    let output = if available >= n { "YES" } else { "NO" };

    println!("{output}");
}
