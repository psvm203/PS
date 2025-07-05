use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let nums: Vec<i32> = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    let [x, y] = [nums[0], nums[1]];
    let a = 100 - x;
    let b = 100 - y;
    let c = 100 - a - b;
    let d = a * b;
    let q = d / 100;
    let r = d % 100;

    println!("{a} {b} {c} {d} {q} {r}\n{} {r}", c + q);
}
