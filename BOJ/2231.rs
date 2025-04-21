use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let decompose = |number: usize| {
        let digits_sum: usize = number
            .to_string()
            .chars()
            .map(|ch| ch as usize - '0' as usize)
            .sum();

        number + digits_sum
    };

    let answer = match (1..n).find(|&x| decompose(x) == n) {
        Some(creator) => creator,
        None => 0,
    };

    print!("{answer}");
}
