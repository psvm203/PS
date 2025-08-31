use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let output = stdin
        .split_ascii_whitespace()
        .find(|name| name.contains("S"))
        .unwrap();

    println!("{output}");
}
