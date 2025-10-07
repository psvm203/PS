use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let mut nums = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>());

    let n = nums.next().unwrap();
    let l = nums.next().unwrap();
    let h = nums.next().unwrap();
    let mut scores: Vec<usize> = nums.collect();
    scores.sort();
    let scores = &scores[l..n - h];

    let sum: f64 = scores.into_iter().map(|x| *x as f64).sum();
    let average = sum / (n - l - h) as f64;

    println!("{:.10}", average);
}
