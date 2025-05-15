use std::collections::*;
use std::convert::*;
use std::io::*;

fn get_degrees_count(
    children: &Vec<Vec<usize>>,
    degrees_counts: &mut Vec<usize>,
    ancestor: usize,
) -> usize {
    if degrees_counts[ancestor] != 0 {
        return degrees_counts[ancestor];
    }

    let mut degrees_count = 1;

    for &child in &children[ancestor] {
        degrees_count += get_degrees_count(children, degrees_counts, child);
    }

    degrees_counts[ancestor] = degrees_count;
    degrees_count
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

    let [n, root, queries_count] = read_ints().try_into().unwrap();

    let mut graph = vec![vec![]; n + 1];

    for _ in 0..n - 1 {
        let [u, v] = read_ints().try_into().unwrap();
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut deque = VecDeque::from([root]);
    let mut visited = vec![false; n + 1];
    visited[root] = true;
    let mut children = vec![vec![]; n + 1];

    while let Some(cur) = deque.pop_front() {
        for &next in &graph[cur] {
            if !visited[next] {
                visited[next] = true;
                children[cur].push(next);
                deque.push_back(next);
            }
        }
    }

    let mut degrees_counts = vec![0; n + 1];

    let output = (0..queries_count)
        .map(|_| get_degrees_count(&children, &mut degrees_counts, read_ints()[0]).to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
