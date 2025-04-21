use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let lines = input.lines();

    let check_right = |line: &str| {
        let mut sides: Vec<usize> = line
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
            .collect();

        sides.sort();

        let (a, b, c) = (sides[0], sides[1], sides[2]);

        let result = if a * a + b * b == c * c {
            "right"
        } else {
            "wrong"
        };

        result
    };

    let answer = lines
        .take_while(|&line| line != "0 0 0")
        .map(|line| check_right(line))
        .collect::<Vec<_>>()
        .join("\n");

    print!("{answer}");
}
