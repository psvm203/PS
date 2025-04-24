use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let texts = input.lines().take_while(|&x| x != "0");

    let check_palindrome = |line: &str| {
        if line.chars().eq(line.chars().rev()) {
            "yes"
        } else {
            "no"
        }
    };

    let output = texts
        .map(|line| check_palindrome(line))
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
