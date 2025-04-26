use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let numbers = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse::<usize>());

    let mut stack = Vec::new();

    for number in numbers {
        if number == 0 {
            stack.pop();
        } else {
            stack.push(number);
        }
    }

    let sum: usize = stack.iter().sum();

    print!("{sum}");
}
