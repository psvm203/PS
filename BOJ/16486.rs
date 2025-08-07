use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let mut nums = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>());

    let [d1, d2] = [nums.next().unwrap(), nums.next().unwrap()];

    let circumference = d1 as f64 * 2.0 + d2 as f64 * 2.0 * 3.141592;

    println!("{circumference}");
}
