use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let words = stdin.split_ascii_whitespace().skip(1);

    let output = words
        .map(|word| match word.chars().count() {
            6..=9 => "yes",
            _ => "no",
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
