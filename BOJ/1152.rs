use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let count = input.split_ascii_whitespace().count();

    print!("{count}");
}
