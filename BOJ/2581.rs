use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let [m, n] = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    let mut is_prime = vec![true; n + 1];
    is_prime[1] = false;

    for i in 2..=n {
        if is_prime[i] {
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    let primes = (m..=n).filter(|x| is_prime[*x]);

    let output = if primes.clone().count() == 0 {
        "-1".to_owned()
    } else {
        format!(
            "{}\n{}",
            primes.clone().sum::<usize>(),
            primes.min().unwrap()
        )
    };

    println!("{output}");
}
