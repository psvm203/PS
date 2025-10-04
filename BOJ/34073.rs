use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let words = stdin.split_ascii_whitespace().skip(1);

    let output = words
        .map(|word| word.to_owned() + "DORO")
        .collect::<Vec<_>>()
        .join(" ");

    println!("{output}");
}
