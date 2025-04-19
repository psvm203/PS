use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let numbers: Vec<usize> = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    let (hour, minute) = (numbers[0], numbers[1]);

    let minute = (hour + 24) * 60 + minute - 45;

    let hour = minute / 60 % 24;
    let minute = minute % 60;

    print!("{hour} {minute}");
}
