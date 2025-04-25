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

    let cmp = |a: &Vec<i32>, b: &Vec<i32>| {
        if a[1] == b[1] {
            a[0].cmp(&b[0])
        } else {
            a[1].cmp(&b[1])
        }
    };

    positions.sort_by(|a, b| cmp(&a, &b));

    let output = positions
        .iter()
        .map(|position| format!("{} {}", position[0], position[1]))
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
