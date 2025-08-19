use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();

    let mut sum = || {
        lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .flat_map(|x| x.parse::<usize>())
            .sum::<usize>()
            .to_string()
    };

    let output = (0..t).map(|_| sum()).collect::<Vec<_>>().join("\n");

    println!("{output}");
}
