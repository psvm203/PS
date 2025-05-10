use std::collections::VecDeque;
use std::convert::TryInto;
use std::io::*;

const INF: usize = 987654321;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let lines = input.lines();

    let mut paths = [0; 101];

    for i in 1..=100 {
        paths[i] = i;
    }

    lines.skip(1).for_each(|line| {
        let [u, v] = line
            .split(' ')
            .flat_map(|x| x.parse::<usize>())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        paths[u] = v;
    });

    let mut dists = [INF; 101];
    dists[1] = 0;
    let mut deque = VecDeque::from([1]);

    while let Some(cur) = deque.pop_front() {
        for dice in 1..=6 {
            let next = cur + dice;

            if next > 100 {
                break;
            }

            let next = paths[next];

            let dist = dists[cur] + 1;

            if dist < dists[next] {
                dists[next] = dist;
                deque.push_back(next);
            }
        }
    }

    print!("{}", dists[100]);
}
