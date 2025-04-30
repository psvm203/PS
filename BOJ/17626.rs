use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut is_square = vec![false; n + 1];
    let mut squares = Vec::new();

    (1..)
        .map(|i| i * i)
        .take_while(|&x| x <= n)
        .for_each(|square| {
            is_square[square] = true;
            squares.push(square);
        });

    if is_square[n] {
        print!("1");
        return;
    }

    let mut is_two_sum = vec![false; n + 1];

    (0..squares.len()).for_each(|i| {
        (i..squares.len())
            .map(|j| squares[i] + squares[j])
            .filter(|&sum| sum < n)
            .for_each(|sum| is_two_sum[sum] = true);
    });

    if is_two_sum[n] {
        print!("2");
        return;
    }

    let is_three = squares
        .iter()
        .find(|&&square| is_two_sum[n - square])
        .is_some();

    if is_three {
        print!("3");
    } else {
        print!("4");
    }
}
