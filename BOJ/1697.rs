use std::collections::VecDeque;
use std::convert::TryInto;
use std::io::*;

const INF: usize = 987654321;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let [n, k] = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    if n >= k {
        print!("{}", n - k);
        return;
    }

    let mut deque = VecDeque::from([k]);
    let mut min_costs = [INF; 100001];
    min_costs[k] = 0;

    while let Some(cur) = deque.pop_front() {
        let mut nexts = Vec::new();

        if cur + 1 <= 100000 {
            nexts.push(cur + 1);
        }

        if cur >= 1 {
            nexts.push(cur - 1);
        }

        if cur % 2 == 0 {
            nexts.push(cur / 2);
        }

        for &next in &nexts {
            let cost = min_costs[cur] + 1;

            if cost < min_costs[next] {
                min_costs[next] = cost;
                deque.push_back(next);
            }
        }
    }

    let min_cost = min_costs[n];
    print!("{min_cost}");
}
