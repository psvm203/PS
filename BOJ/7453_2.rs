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

    let mut sums = vec![];

    for i in 0..n {
        for j in 0..n {
            let sum = vecs[i][0] + vecs[j][1];
            sums.push(sum);
        }
    }

    sums.sort();

    let mut negative_sums = vec![];

    for i in 0..n {
        for j in 0..n {
            let negative_sum = -vecs[i][2] - vecs[j][3];
            negative_sums.push(negative_sum);
        }
    }

    negative_sums.sort();

    let mut i = 0;
    let mut j = 0;
    let mut cases_count: i64 = 0;

    while i < sums.len() && j < negative_sums.len() {
        let a = sums[i];
        let b = negative_sums[j];

        if a < b {
            i += 1;
            continue;
        }

        if a > b {
            j += 1;
            continue;
        }

        let mut len = 0;

        while i < sums.len() && sums[i] == a {
            len += 1;
            i += 1;
        }

        while j < negative_sums.len() && negative_sums[j] == b {
            cases_count += len;
            j += 1;
        }
    }

    println!("{cases_count}");
}
