use std::cmp::Ordering::*;
use std::io::*;

fn find(roots: &mut [usize; 1001], x: usize) -> usize {
    if roots[x] == x {
        return x;
    }

    roots[x] = find(roots, roots[x]);
    roots[x]
}

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

    let mut roots = [0; 1001];
    let mut ranks = [0; 1001];

    (1..=n).for_each(|i| roots[i] = i);

    let mut union = |a, b| {
        let root_a = find(&mut roots, a);
        let root_b = find(&mut roots, b);

        if root_a == root_b {
            return;
        }

        match ranks[root_a].cmp(&ranks[root_b]) {
            Less => roots[root_a] = root_b,
            Greater => roots[root_b] = root_a,
            Equal => {
                roots[root_a] = root_b;
                ranks[root_a] += 1;
            }
        };
    };

    (0..m).for_each(|_| {
        let (u, v) = read_int_pair();
        union(u, v);
    });

    let output = (1..=n).filter(|&i| roots[i] == i).count();

    print!("{output}");
}
