use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let queries = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse::<i32>());

    let mut max_heap = BinaryHeap::new();
    let mut min_heap = BinaryHeap::new();

    let mut process_query = |query| match query {
        x if x < 0 => {
            max_heap.push(query);
            None
        }
        x if x > 0 => {
            min_heap.push(Reverse(query));
            None
        }
        0 => match (max_heap.peek(), min_heap.peek()) {
            (None, None) => Some(0),
            (Some(_), None) => max_heap.pop(),
            (None, Some(_)) => Some(min_heap.pop().unwrap().0),
            (Some(&a), Some(&Reverse(b))) if -a < b => max_heap.pop(),
            (Some(&a), Some(&Reverse(b))) if -a == b => max_heap.pop(),
            (Some(&a), Some(&Reverse(b))) if -a > b => Some(min_heap.pop().unwrap().0),
            _ => None,
        },
        _ => unreachable!(),
    };

    let output = queries
        .filter_map(|query| process_query(query))
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
