use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines();

    let s: String = lines
        .next()
        .unwrap()
        .chars()
        .filter(|ch| !ch.is_digit(10))
        .collect();

    let k = lines.next().unwrap();

    let output = s.contains(k) as usize;

    println!("{output}");
}
