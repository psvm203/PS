use std::io::*;

const MODULO: usize = 1_000_000_000;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.trim().parse().unwrap();

    // memo[i][prev][mask]
    let mut memo = vec![[[0; 1024]; 10]; n];

    for digit in 1..10 {
        let mask = 1 << digit;
        memo[0][digit][mask] = 1;
    }

    for i in 1..n {
        for digit in 0..10 {
            for offset in [-1, 1] {
                let prev = digit as i32 + offset;

                if prev == -1 || prev == 10 {
                    continue;
                }

                let prev = prev as usize;

                for mask in 0..1024 {
                    let next_mask = mask | (1 << digit);
                    memo[i][digit][next_mask] += memo[i - 1][prev][mask];
                    memo[i][digit][next_mask] %= MODULO;
                }
            }
        }
    }

    let mut count = 0;

    for &m in &memo[n - 1] {
        count += m[1023];
        count %= MODULO;
    }

    println!("{count}");
}
