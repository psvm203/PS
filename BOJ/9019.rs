use std::collections::VecDeque;
use std::convert::TryInto;
use std::io::*;

fn double(x: usize) -> usize {
    x * 2 % 10000
}

fn subtract(x: usize) -> usize {
    (x + 9999) % 10000
}

fn left(x: usize) -> usize {
    (x / 1000 + x * 10) % 10000
}

fn right(x: usize) -> usize {
    (x % 10 * 10000 + x) / 10
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let mut read_ints = || -> Vec<usize> {
        lines
            .next()
            .unwrap()
            .split(' ')
            .flat_map(|x| x.parse())
            .collect()
    };

    let test_cases_count = read_ints()[0];

    let operations: [(char, fn(usize) -> usize); 4] =
        [('D', double), ('S', subtract), ('L', left), ('R', right)];

    let solve_test_case = |begin, end| {
        let mut deque = VecDeque::from([begin]);
        let mut trails = ['N'; 10000];
        trails[begin] = 'N';
        let mut prevs = [0; 10000];

        'Outer: while let Some(cur) = deque.pop_front() {
            for &(operator, operation) in &operations {
                let next = operation(cur);

                if trails[next] != 'N' {
                    continue;
                }

                trails[next] = operator;
                prevs[next] = cur;
                deque.push_back(next);

                if next == end {
                    break 'Outer;
                }
            }
        }

        let mut cur = end;
        let mut routes = String::new();

        while cur != begin {
            routes.push(trails[cur]);
            cur = prevs[cur];
        }

        routes.chars().rev().collect::<String>()
    };

    let output = (0..test_cases_count)
        .map(|_| {
            let [begin, end] = read_ints().try_into().unwrap();
            solve_test_case(begin, end).to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
