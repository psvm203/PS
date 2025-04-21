use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let mut read_line = || lines.next().unwrap();

    let word = read_line();
    let i: usize = read_line().parse().unwrap();
    let ch = word.chars().nth(i - 1).unwrap();

    print!("{ch}");
}
