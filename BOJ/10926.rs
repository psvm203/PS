use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let id = input.trim().to_owned();

    let output = id + "??!";

    print!("{output}");
}
