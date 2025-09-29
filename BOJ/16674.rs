use std::io::*;

fn solve(mut n: usize) -> usize {
    let mut counts = [0; 10];

    while n > 0 {
        let digit = n % 10;

        if digit != 2 && digit != 0 && digit != 1 && digit != 8 {
            return 0;
        }

        counts[n % 10] += 1;
        n /= 10;
    }

    if counts[2] == counts[0] && counts[0] == counts[1] && counts[1] == counts[8] {
        return 8;
    }

    if counts[2] * counts[0] * counts[1] * counts[8] == 0 {
        return 1;
    }

    return 2;
}

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.trim().parse().unwrap();

    println!("{}", solve(n));
}
