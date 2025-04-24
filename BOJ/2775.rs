use std::io::*;

fn dp(memo: &mut [[usize; 15]; 15], y: usize, x: usize) -> usize {
    if memo[y][x] != 0 {
        return memo[y][x];
    }

    memo[y][x] = (1..=x).map(|i| dp(memo, y - 1, i)).sum();

    return memo[y][x];
}

fn main() {
    let mut memo = [[0; 15]; 15];
    (1..15).for_each(|i| memo[0][i] = i);

    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let mut read_int = || -> usize { lines.next().unwrap().trim().parse().unwrap() };

    let test_cases_count = read_int();

    let output = (0..test_cases_count)
        .map(|_| (read_int(), read_int()))
        .map(|(y, x)| dp(&mut memo, y, x).to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
