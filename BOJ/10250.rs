use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let calculate_room = |line: &str| {
        let numbers: Vec<usize> = line
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
            .collect();

        let (height, n) = (numbers[0], numbers[2]);

        let y = (n - 1) % height + 1;
        let x = (n - 1) / height + 1;
        let room = y * 100 + x;
        room
    };

    let rooms = input
        .lines()
        .skip(1)
        .map(|line| calculate_room(line).to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{rooms}");
}
