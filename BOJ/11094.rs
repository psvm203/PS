use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let lines = stdin.lines().skip(1);

    let output = lines
        .filter(|line| line.starts_with("Simon says"))
        .map(|line| &line[10..])
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
