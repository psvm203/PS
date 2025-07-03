use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let mut scores: Vec<usize> = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    let a = &mut scores.clone()[0..4];
    let b = &mut scores[4..6];

    a.sort();
    b.sort();

    let max_score = a[1] + a[2] + a[3] + b[1];

    println!("{max_score}");
}
