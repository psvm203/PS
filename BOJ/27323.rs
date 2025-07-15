use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let nums: Vec<usize> = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    let [a, b] = [nums[0], nums[1]];
    let area = a * b;

    println!("{area}");
}
