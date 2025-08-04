use std::collections::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut splitted = stdin.split_ascii_whitespace();

    let n: usize = splitted.next().unwrap().parse().unwrap();

    let map: HashMap<_, _> = (0..n)
        .map(|_| {
            let v = splitted.next().unwrap().to_owned();
            let k = splitted.next().unwrap().chars().next().unwrap();
            (k, v)
        })
        .collect();

    let compressed = splitted.next().unwrap();
    let decompressed: String = compressed.chars().map(|ch| map[&ch].clone()).collect();

    let s: usize = splitted.next().unwrap().parse().unwrap();
    let e: usize = splitted.next().unwrap().parse().unwrap();
    let output = &decompressed[s - 1..e];

    println!("{output}");
}
