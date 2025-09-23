use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let lines = stdin.lines().take_while(|line| *line != "0 0");

    let output = lines
        .map(|line| {
            line.split_ascii_whitespace()
                .flat_map(|x| x.parse::<usize>())
                .sum::<usize>()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
