use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines();

    let mut read_ints = || -> Vec<usize> {
        lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
            .collect()
    };

    let [n, m] = read_ints().try_into().unwrap();
    let s = read_ints();
    let t: Vec<Vec<usize>> = (0..n).map(|_| read_ints()).collect();

    let mut pays = s.clone();
    let mut extend = vec![0; m];
    pays.append(&mut extend);

    for i in 0..n {
        pays[i] -= t[i].iter().sum::<usize>();
    }

    for i in 0..n {
        for j in 0..n + m {
            pays[j] += t[i][j];
        }
    }

    let output = pays
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{output}");
}
