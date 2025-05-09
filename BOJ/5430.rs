use std::collections::VecDeque;
use std::io::*;
use std::str::Lines;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let read_operators = |lines: &mut Lines| -> Vec<_> { lines.next().unwrap().chars().collect() };

    let read_int = |lines: &mut Lines| -> usize { lines.next().unwrap().parse().unwrap() };

    let read_deque = |lines: &mut Lines| -> VecDeque<usize> {
        let string = lines.next().unwrap();
        string[1..string.len() - 1]
            .split(',')
            .flat_map(|x| x.parse())
            .collect()
    };

    let test_cases_count = read_int(&mut lines);

    for _ in 0..test_cases_count {
        let operators = read_operators(&mut lines);
        let _ = read_int(&mut lines);
        let mut deque = read_deque(&mut lines);
        let mut is_reversed = false;
        let mut has_error = false;

        for &operator in &operators {
            match operator {
                'R' => {
                    is_reversed ^= true;
                }
                'D' => match is_reversed {
                    true => {
                        if deque.pop_back().is_none() {
                            has_error = true;
                            break;
                        }
                    }
                    false => {
                        if deque.pop_front().is_none() {
                            has_error = true;
                            break;
                        }
                    }
                },
                _ => unreachable!(),
            }
        }

        let output = if has_error {
            String::from("error")
        } else if is_reversed {
            let nums = deque
                .iter()
                .rev()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",");

            format!("[{nums}]")
        } else {
            let nums = deque
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",");

            format!("[{nums}]")
        };

        println!("{output}");
    }
}
