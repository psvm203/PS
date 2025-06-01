use std::io::*;

fn subvec_sums_of_vec(vec: &Vec<i32>) -> Vec<i32> {
    let prefix_sum: Vec<i32> = vec
        .iter()
        .scan(0, |sum, x| {
            *sum += x;
            Some(*sum)
        })
        .collect();

    let mut sums = vec![];

    for &sum in &prefix_sum {
        sums.push(sum);
    }

    for lo in 1..vec.len() {
        for hi in lo + 1..=vec.len() {
            let sum = prefix_sum[hi - 1] - prefix_sum[lo - 1];
            sums.push(sum);
        }
    }

    sums.sort_unstable();
    sums
}

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

    let t = read_ints()[0];
    let _ = read_ints();
    let a = read_ints();
    let _ = read_ints();
    let b = read_ints();

    let a_sums = subvec_sums_of_vec(&a);
    let b_sums: Vec<_> = subvec_sums_of_vec(&b).into_iter().rev().collect();
    let n = a_sums.len();
    let m = b_sums.len();

    let mut i = 0;
    let mut j = 0;
    let mut count = 0_usize;

    while i < n && j < m {
        let sum = a_sums[i] + b_sums[j];

        if sum < t {
            i += 1;
        } else if sum > t {
            j += 1;
        } else {
            let a_sum = a_sums[i];
            let b_sum = b_sums[j];
            let mut a_count = 0;
            let mut b_count = 0;

            while i < n {
                if a_sums[i] == a_sum {
                    a_count += 1;
                    i += 1;
                } else {
                    break;
                }
            }

            while j < m {
                if b_sums[j] == b_sum {
                    b_count += 1;
                    j += 1;
                } else {
                    break;
                }
            }

            count += a_count as usize * b_count as usize;
        }
    }

    println!("{count}");
}
