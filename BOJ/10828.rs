use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let queries = input
        .lines()
        .skip(1)
        .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>());

    let mut stack = Vec::new();

    let output = queries
        .filter_map(|query| {
            let read_int = || -> i32 { query[1].parse().unwrap() };

            match query[0] {
                "push" => {
                    stack.push(read_int());
                    None
                }
                "pop" => Some(stack.pop().unwrap_or(-1)),
                "size" => Some(stack.len() as i32),
                "empty" => Some(stack.is_empty() as i32),
                "top" => Some(*stack.last().unwrap_or(&-1)),
                _ => None,
            }
        })
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
