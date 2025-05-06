use std::collections::HashMap;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let nums: Vec<i32> = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse())
        .collect();

    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    sorted_nums.dedup();

    let mut map = HashMap::new();

    for (i, num) in sorted_nums.iter().enumerate() {
        map.insert(num, i);
    }

    let output = nums
        .iter()
        .flat_map(|x| map.get(x))
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    print!("{output}");
}
