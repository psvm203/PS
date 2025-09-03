use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let output = stdin
        .lines()
        .enumerate()
        .skip(1)
        .map(|(i, s)| format!("{i}. {s}"))
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
