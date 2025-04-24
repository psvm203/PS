use std::io::*;
use std::iter::successors;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let factorize_by_five = |x| {
        successors(Some(5usize), |&product| product.checked_mul(5))
            .take_while(|&factor| x % factor == 0)
            .count()
    };

    let output: usize = (5..=n).step_by(5).map(|x| factorize_by_five(x)).sum();

    print!("{output}");
}
