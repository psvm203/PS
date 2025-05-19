use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let count = stdin.trim().chars().count();

    print!("{count}");
}
