use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let numbers: Vec<i32> = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    let (a, b, c) = (numbers[0], numbers[1], numbers[2]);

    let line_1 = a + b - c;
    let line_2 = format!("{a}{b}").parse::<i32>().unwrap() - c;

    print!("{line_1}\n{line_2}");
}
