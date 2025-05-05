use std::collections::BinaryHeap;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let queries = input.lines().skip(1).flat_map(|x| x.parse::<usize>());

    let mut heap = BinaryHeap::new();

    let mut process_query = |query| match query {
        0 => heap.pop().or(Some(0)),
        _ => {
            heap.push(query);
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
