use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let nums = stdin
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse())
        .collect::<Vec<i32>>();

    let mut output = 0;
    let mut prev_left = 2;
    let mut prev_right = 2;

    for chunk in nums.chunks(2) {
        let [left, right] = chunk.try_into().unwrap();
        if left != 0 && left == right {
            output += 1;
        }
        if left != 0 && left == prev_left {
            output += 1;
        }
        if right != 0 && right == prev_right {
            output += 1;
        }
        prev_left = left;
        prev_right = right;
    }

    println!("{output}");
}
