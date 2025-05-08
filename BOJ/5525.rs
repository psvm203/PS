use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let mut read_int = || -> usize { lines.next().unwrap().parse().unwrap() };

    let n = read_int();
    let m = read_int();
    let s: Vec<_> = lines.next().unwrap().chars().collect();

    let mut memo = vec![0; m];

    if s[0] == 'I' {
        memo[0] = 1;
    }

    let mut prev = s[0];

    for i in 1..m {
        if s[i] != prev {
            memo[i] = memo[i - 1] + 1;
        } else if s[i] == 'I' {
            memo[i] = 1;
        } else {
            memo[i] = 0;
        }

        prev = s[i];
    }

    let mut count = 0;

    for &m in &memo {
        if m % 2 == 1 && (m - 1) / 2 >= n {
            count += 1;
        }
    }

    print!("{count}");
}
