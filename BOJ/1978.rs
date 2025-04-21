use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let numbers = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse::<usize>());

    let mut is_prime = [true; 1001];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..32 {
        if is_prime[i] {
            for j in (i * i..1001).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    let primes_count = numbers.filter(|&number| is_prime[number]).count();

    print!("{primes_count}");
}
