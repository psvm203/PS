use std::collections::HashSet;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let queries = input
        .lines()
        .skip(1)
        .map(|x| x.split_ascii_whitespace().collect::<Vec<_>>());

    let mut set = HashSet::new();

    let mut process_query = |query: Vec<&str>| {
        let read_int = || -> usize { query[1].parse().unwrap() };

        match query[0] {
            "add" => {
                set.insert(read_int());
                None
            }
            "remove" => {
                set.remove(&read_int());
                None
            }
            "check" => Some(set.contains(&read_int()) as usize),
            "toggle" => {
                let x = read_int();
                if !set.insert(x) {
                    set.remove(&x);
                }
                None
            }
            "all" => {
                set = (1..21).collect();
                None
            }
            "empty" => {
                set.clear();
                None
            }
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
