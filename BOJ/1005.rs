use std::convert::TryInto;
use std::io::*;

const INF: usize = 987654321;

fn dfs(
    costs: &Vec<usize>,
    requirements: &Vec<Vec<usize>>,
    dists: &mut Vec<usize>,
    x: usize,
) -> usize {
    let mut dist = 0;

    for &requirement in &requirements[x] {
        if dists[requirement] == INF {
            dfs(costs, requirements, dists, requirement);
        }

        dist = dist.max(dists[requirement]);
    }

    dists[x] = costs[x] + dist;
    return dists[x];
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

    let t = read_ints()[0];

    for _ in 0..t {
        let [n, k] = read_ints().try_into().unwrap();
        let costs = read_ints();

        let mut requirements = vec![vec![]; n];

        for _ in 0..k {
            let [x, y] = read_ints().try_into().unwrap();

            requirements[y - 1].push(x - 1);
        }

        let mut dists = vec![INF; n];

        let w = read_ints()[0] - 1;
        let time = dfs(&costs, &requirements, &mut dists, w);

        println!("{time}")
    }
}
