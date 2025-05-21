use std::collections::*;
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
            .flat_map(|x| x.parse())
            .collect()
    };

    let n = read_ints()[0];
    let nums = read_ints();
    let queries_count = read_ints()[0];

    let mut is_palindrome = vec![vec![false; n + 1]; n];

    for i in 0..n {
        let mut lo = i;
        let mut hi = i + 1;

        while nums.get(lo) == nums.get(hi - 1) {
            is_palindrome[lo][hi] = true;

            if lo == 0 {
                break;
            }

            lo -= 1;
            hi += 1;
        }
    }

    for i in 1..n {
        let mut lo = i - 1;
        let mut hi = i + 1;

        while nums.get(lo) == nums.get(hi - 1) {
            is_palindrome[lo][hi] = true;

            if lo == 0 {
                break;
            }

            lo -= 1;
            hi += 1;
        }
    }

    let output = (0..queries_count)
        .map(|_| {
            let [s, e] = read_ints().try_into().unwrap();
            (is_palindrome[s - 1][e] as usize).to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
