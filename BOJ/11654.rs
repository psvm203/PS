use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let code = input.chars().next().unwrap() as usize;

    print!("{code}");
}
