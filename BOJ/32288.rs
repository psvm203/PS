use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let nickname = stdin.split_ascii_whitespace().nth(1).unwrap();

    let output: String = nickname
        .chars()
        .map(|ch| if ch == 'l' { 'L' } else { 'i' })
        .collect();

    println!("{output}");
}
