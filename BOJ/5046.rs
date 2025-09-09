use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines();

    let [n, b, h, _w] = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    let min = (0..h)
        .map(|_| {
            let p: usize = lines.next().unwrap().parse().unwrap();
            let a = lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .flat_map(|x| x.parse::<usize>());

            if a.max().unwrap() < n {
                987654321
            } else {
                n * p
            }
        })
        .min()
        .unwrap();

    let m: usize = min;

    let output = if b < min {
        "stay home".to_owned()
    } else {
        m.to_string()
    };

    println!("{output}");
}
