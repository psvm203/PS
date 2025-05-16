use std::convert::*;
use std::io::*;

fn find(parents: &mut Vec<usize>, x: usize) -> usize {
    if parents[x] == x {
        return x;
    }

    parents[x] = find(parents, parents[x]);
    parents[x]
}

fn union(parents: &mut Vec<usize>, ranks: &mut Vec<usize>, a: usize, b: usize) {
    let a = find(parents, a);
    let b = find(parents, b);

    if a == b {
        return;
    }

    if ranks[a] < ranks[b] {
        parents[a] = b;
    } else if ranks[a] > ranks[b] {
        parents[b] = a;
    } else {
        parents[b] = a;
        ranks[a] += 1;
    }
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

    let [vertices_count, edges_count] = read_ints().try_into().unwrap();
    let mut edges: Vec<_> = (0..edges_count).map(|_| read_ints()).collect();
    edges.sort_unstable_by(|a, b| a[2].cmp(&b[2]));

    let mut parents: Vec<_> = (0..=vertices_count).collect();
    let mut ranks = vec![0; vertices_count + 1];
    let mut costs = vec![];

    for edge in edges {
        let [u, v, cost] = edge.try_into().unwrap();

        if find(&mut parents, u) != find(&mut parents, v) {
            union(&mut parents, &mut ranks, u, v);
            costs.push(cost);
        }
    }

    let costs_sum: usize = costs.iter().sum();
    let max_cost = costs.iter().max().unwrap();
    let result: usize = costs_sum - max_cost;

    print!("{result}");
}
