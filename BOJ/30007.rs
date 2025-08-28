use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let lines = stdin.lines().skip(1);

    let output = lines
        .map(|line| {
            let [a, b, x] = line
                .split_ascii_whitespace()
                .flat_map(|x| x.parse())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap();

            a * (x - 1) + b
        })
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
