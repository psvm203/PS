use std::collections::BTreeMap;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut splitted = input.split_ascii_whitespace();

    let mut read_next = || splitted.next().unwrap();

    let test_cases_count: usize = read_next().parse().unwrap();

    let mut solve_test_case = || {
        let n: usize = read_next().parse().unwrap();
        let mut counts = BTreeMap::new();

        (0..n).for_each(|_| {
            let operator = read_next();
            let operand: i32 = read_next().parse().unwrap();

            match (operator, operand) {
                ("I", _) => {
                    counts
                        .entry(operand)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
                ("D", 1) => {
                    if let Some(mut entry) = counts.last_entry() {
                        *entry.get_mut() -= 1;

                        if *entry.get() == 0 {
                            entry.remove();
                        }
                    }
                }
                ("D", -1) => {
                    if let Some(mut entry) = counts.first_entry() {
                        *entry.get_mut() -= 1;

                        if *entry.get() == 0 {
                            entry.remove();
                        }
                    }
                }
                _ => unreachable!(),
            }
        });

        if counts.is_empty() {
            String::from("EMPTY")
        } else {
            let max = *counts.last_entry().unwrap().key();
            let min = *counts.first_entry().unwrap().key();
            format!("{max} {min}")
        }
    };

    let output = (0..test_cases_count)
        .map(|_| solve_test_case())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
