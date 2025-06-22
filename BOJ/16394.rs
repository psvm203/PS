use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let n: usize = stdin.trim().parse().unwrap();

    print!("{}", n - 1946);
}
