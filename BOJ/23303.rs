use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let stdin = stdin.to_lowercase();

    let output = if stdin.contains("d2") {
        "D2"
    } else {
        "unrated "
    };

    println!("{output}");
}
