use std::convert::*;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let [x, y] = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<i32>())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let quadrant = match (y > 0, x > 0) {
        (true, true) => 1,
        (true, false) => 2,
        (false, false) => 3,
        (false, true) => 4,
    };

    print!("{quadrant}");
}
