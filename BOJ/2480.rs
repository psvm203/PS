use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let mut dice: Vec<usize> = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    dice.sort();

    let prize = if dice[0] == dice[2] {
        10000 + dice[0] * 1000
    } else if dice[0] == dice[1] || dice[1] == dice[2] {
        1000 + dice[1] * 100
    } else {
        dice[2] * 100
    };

    println!("{prize}");
}
