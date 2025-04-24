use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let (n, k) = {
        let vec: Vec<usize> = input
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
            .collect();

        (vec[0], vec[1])
    };

    let calc_factorial = |x| (1..=x).fold(1, |acc, num| acc * num);

    let binomial_coefficient = calc_factorial(n) / calc_factorial(k) / calc_factorial(n - k);

    print!("{binomial_coefficient}");
}
