use std::cmp::Ordering::*;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let numbers: Vec<i32> = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    let (a, b) = (numbers[0], numbers[1]);

    let cmp = match a.cmp(&b) {
        Greater => ">",
        Less => "<",
        Equal => "==",
    };

    print!("{cmp}");
}
