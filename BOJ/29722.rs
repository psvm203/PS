use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let [mut y, mut m, mut d, n] = stdin
        .split(['-', '\n'])
        .take(4)
        .flat_map(|x| x.parse())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    d += n;

    while d > 30 {
        d -= 30;
        m += 1;
    }

    while m > 12 {
        m -= 12;
        y += 1;
    }

    println!("{y}-{:0>2}-{:0>2}", m, d);
}
