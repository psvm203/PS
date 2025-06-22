use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let lines = stdin.lines().skip(1);

    let output = lines
        .map(|line| line.to_lowercase())
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
