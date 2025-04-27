use std::collections::VecDeque;
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

    let test_cases_count = read_ints()[0];

    let mut calculate_order = || {
        let m = read_ints()[1];

        let mut priorities: Vec<_> = read_ints();
        let mut deque: VecDeque<(_, _)> = priorities.iter().copied().enumerate().collect();

        priorities.sort_unstable();

        let mut print_document = || {
            let mut front = deque.pop_front().unwrap();

            while front.1 != *priorities.last().unwrap() {
                deque.push_back(front);
                front = deque.pop_front().unwrap();
            }

            priorities.pop();
            front.0
        };

        (1..)
            .map(|i| (i, print_document()))
            .find(|&(_, document)| document == m)
            .unwrap()
            .0
            .to_string()
    };

    let orders = (0..test_cases_count)
        .map(|_| calculate_order())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{orders}");
}
