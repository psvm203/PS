use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let mut positions: Vec<Vec<i32>> = input
        .lines()
        .skip(1)
        .map(|line| {
            line.split_ascii_whitespace()
                .flat_map(|x| x.parse())
                .collect()
        })
        .collect();

    positions.sort();

    let output = positions
        .iter()
        .map(|position| format!("{} {}", position[0], position[1]))
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
