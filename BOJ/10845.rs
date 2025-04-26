use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let queries = input
        .lines()
        .skip(1)
        .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>());

    let mut queue = VecDeque::new();

    let mut process_query = |query: Vec<&str>| {
        let read_int = || -> i32 { query[1].parse().unwrap() };

        match query[0] {
            "push" => {
                queue.push_back(read_int());
                None
            }
            "pop" => Some(queue.pop_front().unwrap_or(-1)),
            "size" => Some(queue.len() as i32),
            "empty" => Some(queue.is_empty() as i32),
            "front" => Some(*queue.front().unwrap_or(&-1)),
            "back" => Some(*queue.back().unwrap_or(&-1)),
            _ => None,
        }
    };

    let output = queries
        .filter_map(|query| process_query(query))
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
