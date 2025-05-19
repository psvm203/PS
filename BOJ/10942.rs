use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines();

    let mut read_ints = || -> Vec<usize> {
        lines
            .next()
            .unwrap()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect()
    };

    let n = read_ints()[0];
    let nums = read_ints();
    let queries_count = read_ints()[0];

    let mut memo = vec![vec![false; n]; n];

    for i in 0..n {
        memo[i][i] = true;
    }

    for i in 1..n {
        if nums[i - 1] == nums[i] {
            memo[i - 1][i] = true;
        }
    }

    for len in 3..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            if nums[i] == nums[j] && memo[i + 1][j - 1] {
                memo[i][j] = true;
            }
        }
    }

    let output = (0..queries_count)
        .map(|_| {
            let [s, e] = read_ints().try_into().unwrap();
            (memo[s - 1][e - 1] as usize).to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
