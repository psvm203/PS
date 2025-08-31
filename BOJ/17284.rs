use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let output = 5000
        - stdin
            .split_ascii_whitespace()
            .map(|pressed| match pressed {
                "1" => 500,
                "2" => 800,
                _ => 1000,
            })
            .sum::<usize>();

    println!("{output}");
}
