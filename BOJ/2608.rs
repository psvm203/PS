use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let computers_count: usize = lines.next().unwrap().parse().unwrap();

    let network_set = lines.skip(1).map(|line| {
        line.split_ascii_whitespace()
            .flat_map(|x| x.parse::<usize>())
            .collect::<Vec<_>>()
    });

    let mut edges: Vec<Vec<usize>> = vec![vec![]; computers_count + 1];

    network_set.for_each(|network| {
        let (a, b) = (network[0], network[1]);
        edges[a].push(b);
        edges[b].push(a);
    });

    let mut deque = VecDeque::from([1]);
    let mut is_infected = vec![false; computers_count + 1];

    while let Some(cur) = deque.pop_front() {
        is_infected[cur] = true;

        edges[cur]
            .iter()
            .filter(|&x| !is_infected[*x])
            .for_each(|next| {
                deque.push_back(*next);
            });
    }

    let count = is_infected.iter().skip(2).filter(|&x| *x).count();

    print!("{count}");
}
