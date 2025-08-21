use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();
    lines.next();

    let candy = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>());

    let output = if candy.sum::<usize>() >= t {
        "Padaeng_i Happy"
    } else {
        "Padaeng_i Cry"
    };

    println!("{output}");
}
