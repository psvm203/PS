use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let color = stdin
        .split_ascii_whitespace()
        .filter(|s| *s == "w" || *s == "b" || *s == "g")
        .next()
        .unwrap();

    let output = match color {
        "w" => "chunbae",
        "b" => "nabi",
        _ => "yeongcheol",
    }
    .to_owned();

    println!("{output}");
}
