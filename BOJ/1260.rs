use std::collections::*;
use std::convert::TryInto;
use std::io::*;

fn dfs(graph: &Vec<BTreeSet<usize>>, visited: &mut Vec<bool>, cur: usize) {
    visited[cur] = true;

    print!("{cur} ");

    graph[cur].iter().for_each(|&next| {
        if !visited[next] {
            dfs(graph, visited, next);
        }
    });
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let mut read_ints = || -> Vec<usize> {
        lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
            .collect()
    };

    let [verticies_count, edges_count, starting_vertex] = read_ints().try_into().unwrap();
    let edges = (0..edges_count).map(|_| read_ints());

    let mut graph = vec![BTreeSet::new(); verticies_count + 1];

    edges.for_each(|edge| {
        let (a, b) = (edge[0], edge[1]);
        graph[a].insert(b);
        graph[b].insert(a);
    });

    let mut visited = vec![false; verticies_count + 1];

    dfs(&graph, &mut visited, starting_vertex);
    println!();

    visited.fill(false);
    visited[starting_vertex] = true;

    let mut deque = VecDeque::from([starting_vertex]);

    while let Some(cur) = deque.pop_front() {
        print!("{cur} ");

        graph[cur].iter().for_each(|&next| {
            if !visited[next] {
                visited[next] = true;
                deque.push_back(next);
            }
        });
    }
}
