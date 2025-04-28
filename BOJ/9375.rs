use std::collections::HashMap;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let test_cases_count: usize = lines.next().unwrap().parse().unwrap();

    let mut solve_test_case = || {
        let n: usize = lines.next().unwrap().parse().unwrap();

        let mut category_counts = HashMap::new();

        (0..n).for_each(|_| {
            let category = lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .nth(1)
                .unwrap();

            *category_counts.entry(category).or_insert(0) += 1;
        });

        category_counts
            .values()
            .fold(1, |acc, count| acc * (count + 1))
            - 1
    };

    let output = (0..test_cases_count)
        .map(|_| solve_test_case().to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
