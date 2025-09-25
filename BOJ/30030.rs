use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let b: usize = stdin.trim().parse().unwrap();

    let output = b / 11 * 10;

    println!("{output}");
}
