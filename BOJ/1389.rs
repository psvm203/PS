use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::*;

const INF: usize = 987654321;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let mut read_int_pair = || {
        let vec: Vec<usize> = lines
            .next()
            .unwrap()
            .split(' ')
            .flat_map(|x| x.parse())
            .collect();

        (vec[0], vec[1])
    };

    let (n, m) = read_int_pair();

    let mut graph = vec![Vec::new(); n + 1];

    (0..m).for_each(|_| {
        let (u, v) = read_int_pair();
        graph[u].push(v);
        graph[v].push(u);
    });

    let mut min_bacon_number = INF;
    let mut min_bacon_number_person = 0;

    let calc_bacon_number = |x| {
        let mut heap = BinaryHeap::from([Reverse((0, x))]);
        let mut min_costs = vec![INF; n + 1];
        min_costs[x] = 0;

        while let Some(Reverse(tuple)) = heap.pop() {
            let (cost, cur) = tuple;

            for &next in &graph[cur] {
                if cost + 1 >= min_costs[next] {
                    continue;
                }

                heap.push(Reverse((cost + 1, next)));
                min_costs[next] = cost + 1;
            }
        }

        (&min_costs[1..]).iter().sum()
    };

    (1..=n).for_each(|i| {
        let bacon_number = calc_bacon_number(i);

        if bacon_number < min_bacon_number {
            min_bacon_number = bacon_number;
            min_bacon_number_person = i;
        }
    });

    print!("{min_bacon_number_person}");
}
