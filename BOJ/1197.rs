use std::convert::*;
use std::io::*;

fn find(parents: &mut Vec<usize>, x: usize) -> usize {
    if parents[x] != x {
        parents[x] = find(parents, parents[x]);
    }

    parents[x]
}

fn union(parents: &mut Vec<usize>, a: usize, b: usize) {
    let a = find(parents, a);
    let b = find(parents, b);
    parents[b] = parents[a];
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let mut read_ints = || -> Vec<i32> {
        lines
            .next()
            .unwrap()
            .split(' ')
            .flat_map(|x| x.parse())
            .collect()
    };

    let [verticies_count, edges_count] = read_ints().try_into().unwrap();

    let mut edges: Vec<_> = (0..edges_count)
        .map(|_| {
            let [u, v, cost] = read_ints().try_into().unwrap();
            [cost, u, v]
        })
        .collect();

    edges.sort_unstable();

    let mut parents: Vec<_> = (0..=verticies_count as usize).collect();
    let mut result = 0;

    for &[cost, u, v] in &edges {
        let u = u as usize;
        let v = v as usize;

        if find(&mut parents, u) != find(&mut parents, v) {
            union(&mut parents, u, v);
            result += cost;
        }
    }

    print!("{result}");
}
