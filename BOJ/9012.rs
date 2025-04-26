use std::io::*;

fn is_valid(line: &str) -> bool {
    let mut stack = Vec::new();

    for chr in line.chars() {
        match chr {
            '(' => stack.push(false),
            ')' if stack.pop() == None => return false,
            _ => {}
        }
    }

    stack.is_empty()
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let lines = input.lines().skip(1);

    let output = lines
        .map(|line| if is_valid(line) { "YES" } else { "NO" })
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
