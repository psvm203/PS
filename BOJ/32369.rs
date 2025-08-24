use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let [n, a, b] = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    let mut x = 1;
    let mut y = 1;

    for _ in 0..n {
        x += a;
        y += b;

        if x < y {
            let tmp = x;
            x = y;
            y = tmp;
        }

        if x == y {
            y -= 1;
        }
    }

    println!("{x} {y}");
}
