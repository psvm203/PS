use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let nums: Vec<usize> = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    let [t, s] = [nums[0], nums[1]];
    let is_lunch = 12 <= t && t <= 16;
    let output = if s == 1 || !is_lunch { 280 } else { 320 };

    println!("{output}");
}
