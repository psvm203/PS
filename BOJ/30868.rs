use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let test_cases = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .skip(1);

    let output = test_cases
        .map(|x| format!("{}{}", "++++ ".repeat(x / 5), "|".repeat(x % 5)))
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
