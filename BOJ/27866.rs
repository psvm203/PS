use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let word = lines.next().unwrap();
    let i: usize = lines.next().unwrap().parse().unwrap();
    let ch = word.chars().nth(i - 1).unwrap();

    print!("{ch}");
}
