use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let nums: Vec<i32> = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    let [h, i, a, r, c] = [nums[0], nums[1], nums[2], nums[3], nums[4]];

    let output = h * i - a * r * c;

    println!("{output}");
}
