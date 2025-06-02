use std::convert::*;
use std::io::*;

fn find(roots: &mut Vec<usize>, x: usize) -> usize {
    if roots[x] == x {
        return x;
    }

    roots[x] = find(roots, roots[x]);
    return roots[x];
}

fn union(roots: &mut Vec<usize>, ranks: &mut Vec<usize>, a: usize, b: usize) {
    let root_a = find(roots, a);
    let root_b = find(roots, b);

    if root_a == root_b {
        return;
    }

    if ranks[a] < ranks[b] {
        roots[a] = b;
    } else {
        roots[b] = a;
    }

    if ranks[root_a] == ranks[root_b] {
        ranks[root_a] += 1;
    }
}

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines();

    let [r, c] = lines
        .next()
        .unwrap()
        .split(' ')
        .flat_map(|x| x.parse())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    let board: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut roots: Vec<_> = (0..r * c).collect();
    let mut ranks = vec![0; r * c];

    for y in 0..r {
        for x in 0..c {
            let [dy, dx] = match board[y][x] {
                'U' => [-1, 0],
                'D' => [1, 0],
                'L' => [0, -1],
                'R' => [0, 1],
                _ => unreachable!(),
            };

            let ny = (y as i32 + dy) as usize;
            let nx = (x as i32 + dx) as usize;

            let cur = find(&mut roots, y * c + x);
            let next = find(&mut roots, ny * c + nx);

            union(&mut roots, &mut ranks, cur, next);
        }
    }

    let mut roots_vec: Vec<_> = (0..r * c).map(|i| find(&mut roots, i)).collect();

    roots_vec.sort_unstable();
    roots_vec.dedup();

    let count = roots_vec.len();

    println!("{count}");
}
