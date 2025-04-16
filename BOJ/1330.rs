use std::cmp::Ordering::*;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let numbers: Vec<i32> = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    let cmp = match numbers[0].cmp(&numbers[1]) {
        Greater => ">",
        Less => "<",
        Equal => "==",
    };

    print!("{cmp}");
}
