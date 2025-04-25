use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let (r, c) = {
        let vec: Vec<usize> = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
            .collect();

        (vec[0], vec[1])
    };

    let board: Vec<&str> = lines.collect();

    let calc_rewrite_cost = |y, x| {
        let cost: usize = board
            .iter()
            .skip(y)
            .take(8)
            .enumerate()
            .map(|(i, line)| -> usize {
                line.chars()
                    .skip(x)
                    .take(8)
                    .enumerate()
                    .map(|(j, ch)| {
                        let target = if (i + j) % 2 == 0 { 'W' } else { 'B' };
                        (ch == target) as usize
                    })
                    .sum()
            })
            .sum();

        cost.min(64 - cost)
    };

    let min_rewrite_cost = (0..=r - 8)
        .flat_map(|y| (0..=c - 8).map(move |x| calc_rewrite_cost(y, x)))
        .min()
        .unwrap();

    print!("{min_rewrite_cost}");
}
