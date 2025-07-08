use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut splitted = stdin.split_ascii_whitespace();

    let n = splitted.next().unwrap();
    let b: u32 = splitted.next().unwrap().parse().unwrap();

    let mut result = 0;

    for ch in n.chars() {
        result *= b;
        result += ch.to_digit(b).unwrap();
    }

    println!("{result}");
}
