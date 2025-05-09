use std::convert::TryInto;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let [n, r, c] = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let mut len = 1 << (n - 1);
    let mut y = 0;
    let mut x = 0;
    let mut order = 0;
    let mut dist = len * len * 2;

    while len >= 1 {
        if y + len <= r {
            y += len;
            order += dist;
        }

        if x + len <= c {
            x += len;
            order += dist / 2;
        }

        len /= 2;
        dist /= 4;
    }

    print!("{order}");
}
