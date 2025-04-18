use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let scale = input.trim();

    let answer = if scale == "1 2 3 4 5 6 7 8" {
        "ascending"
    } else if scale == "8 7 6 5 4 3 2 1" {
        "descending"
    } else {
        "mixed"
    };

    print!("{answer}");
}
