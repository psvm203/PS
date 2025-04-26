use std::io::*;

fn is_balanced(line: &str) -> bool {
    let mut stack = Vec::new();

    for chr in line.chars().filter(|&x| "()[]".contains(x)) {
        match chr {
            '(' | '[' => stack.push(chr),
            ')' if stack.pop() != Some('(') => return false,
            ']' if stack.pop() != Some('[') => return false,
            _ => {}
        }
    }

    stack.is_empty()
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let lines = input.lines().take_while(|line| *line != ".");

    let output = lines
        .map(|line| if is_balanced(line) { "yes" } else { "no" })
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
