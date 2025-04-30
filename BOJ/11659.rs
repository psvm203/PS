use std::convert::TryInto;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let mut read_ints = || -> Vec<usize> {
        lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
            .collect()
    };

    let [n, m] = read_ints().try_into().unwrap();

    let mut numbers = vec![0];
    numbers.append(&mut read_ints());

    let queries: Vec<_> = (0..m).map(|_| read_ints()).collect();

    let prefix_sum: Vec<_> = numbers
        .iter()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect();

    let sums = queries
        .iter()
        .map(|query| {
            let (i, j) = (query[0], query[1]);
            (prefix_sum[j] - prefix_sum[i - 1]).to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    print!("{sums}");
}
