use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let queries = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse::<usize>());

    let mut heap = BinaryHeap::new();

    let mut process_query = |query| {
        if query == 0 {
            Some(heap.pop().unwrap_or(Reverse(0)).0)
        } else {
            heap.push(Reverse(query));
            None
        }
    };

    let output = queries
        .filter_map(|query| process_query(query))
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
