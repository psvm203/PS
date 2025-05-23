use std::collections::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines();

    let mut read_ints = || -> Vec<i32> {
        lines
            .next()
            .unwrap()
            .split(' ')
            .flat_map(|x| x.parse())
            .collect()
    };

    let n = read_ints()[0] as usize;
    let vecs: Vec<Vec<_>> = (0..n).map(|_| read_ints()).collect();

    let mut counts = HashMap::new();

    for i in 0..n {
        for j in 0..n {
            let sum = vecs[i][0] + vecs[j][1];
            counts
                .entry(sum)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    let mut cases_count: i64 = 0;

    for i in 0..n {
        for j in 0..n {
            let sum = -vecs[i][2] - vecs[j][3];

            if let Some(count) = counts.get(&sum) {
                cases_count += count;
            }
        }
    }

    println!("{cases_count}");
}
