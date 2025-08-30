use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let lines = stdin.lines().skip(1);

    let output = lines
        .map(|line| {
            let [v, e] = line
                .split_ascii_whitespace()
                .flat_map(|x| x.parse())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap();

            (e - v + 2).to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
