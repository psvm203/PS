use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let mut nums = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>());

    let a = nums.next().unwrap();
    let b = nums.next().unwrap();
    let c = nums.next().unwrap();

    let output = if a == 0 {
        c * c - b
    } else if b == 0 {
        c * c - a
    } else {
        ((a + b) as f64).sqrt() as usize
    };

    println!("{output}");
}
