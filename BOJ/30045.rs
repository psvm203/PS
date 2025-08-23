use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let words = stdin.split_ascii_whitespace().skip(1);

    let output = words
        .filter(|word| word.contains("01") || word.contains("OI"))
        .count();

    println!("{output}");
}
