use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.trim().parse().unwrap();

    let count = (n - 2) * (n - 1) * n / 6;

    println!("{count}\n3");
}
