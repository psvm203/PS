use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let nums: Vec<i32> = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    let difference = (nums[0] as i64 - nums[1] as i64).abs();

    print!("{difference}");
}
