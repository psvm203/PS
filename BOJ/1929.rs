use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let (m, n) = {
        let vec: Vec<usize> = input
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
            .collect();

        (vec[0], vec[1])
    };

    let mut is_prime = vec![true; n + 1];

    is_prime[1] = false;

    (2..=n).for_each(|i| {
        if is_prime[i] {
            (i * i..=n).step_by(i).for_each(|j| {
                is_prime[j] = false;
            });
        }
    });

    let primes = (m..=n)
        .filter(|&x| is_prime[x])
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{primes}");
}
