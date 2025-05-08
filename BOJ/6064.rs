use std::convert::TryInto;
use std::io::*;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let mut read_ints = || -> Vec<usize> {
        lines
            .next()
            .unwrap()
            .split(' ')
            .flat_map(|x| x.parse())
            .collect()
    };

    let test_cases_count = read_ints()[0];

    let mut calc_year = || {
        let [m, n, x, y] = read_ints().try_into().unwrap();

        for year in (x..=lcm(m, n)).step_by(m) {
            if year >= y && (year - y) % n == 0 {
                return year as i32;
            }
        }

        -1
    };

    let output = (0..test_cases_count)
        .map(|_| calc_year().to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
