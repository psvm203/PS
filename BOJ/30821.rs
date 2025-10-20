use std::io::*;

fn choose(memo: &mut [[usize; 6]; 101], n: usize, r: usize) -> usize {
    if n < r * 2 {
        return choose(memo, n, n - r);
    }

    if r == 0 {
        return 1;
    }

    if memo[n][r] != 0 {
        return memo[n][r];
    }

    memo[n][r] = choose(memo, n - 1, r) + choose(memo, n - 1, r - 1);
    memo[n][r]
}

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.trim().parse().unwrap();

    let mut memo = [[0; 6]; 101];

    let output = choose(&mut memo, n, 5);

    println!("{output}");
}
