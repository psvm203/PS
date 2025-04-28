use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let mut times: Vec<usize> = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse())
        .collect();

    times.sort_unstable_by(|a, b| b.cmp(a));

    let times_sum: usize = times
        .iter()
        .enumerate()
        .map(|(i, time)| (i + 1) * time)
        .sum();

    print!("{times_sum}");
}
