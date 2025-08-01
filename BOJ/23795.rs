use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let sum: usize = stdin
        .split_ascii_whitespace()
        .take_while(|&x| x != "-1")
        .flat_map(|x| x.parse::<usize>())
        .sum();

    println!("{sum}");
}
