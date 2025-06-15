use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let lines = stdin.lines().take_while(|&line| line != "***");

    let output = lines
        .map(|line| line.chars().rev().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
