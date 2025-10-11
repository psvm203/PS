use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.trim().parse().unwrap();

    let output = if n == 2 { 3 } else { n };

    println!("{output}");
}
