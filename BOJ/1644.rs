use std::io::*;

fn calc_primes(max: usize) -> Vec<usize> {
    let mut is_prime = vec![true; max + 1];

    for i in 2..=((max as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i..=max).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    let mut primes = vec![];

    for i in 2..=max {
        if is_prime[i] {
            primes.push(i);
        }
    }

    primes
}

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.trim().parse().unwrap();

    let primes = calc_primes(n);

    let mut lo = 0;
    let mut hi = 0;
    let mut sum = 0;
    let mut cases_count = 0;

    loop {
        if sum < n {
            hi += 1;

            if hi > primes.len() {
                break;
            }

            sum += primes[hi - 1];
        } else {
            lo += 1;
            sum -= primes[lo - 1];
        }

        if sum == n {
            cases_count += 1;
        }
    }

    println!("{cases_count}");
}
