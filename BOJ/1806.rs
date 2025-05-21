use std::convert::*;
use std::io::*;

const INF: usize = 987654321;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines();

    let mut read_ints = || -> Vec<usize> {
        lines
            .next()
            .unwrap()
            .split(' ')
            .flat_map(|x| x.parse())
            .collect()
    };

    let [n, s] = read_ints().try_into().unwrap();
    let nums = read_ints();

    let mut lo = 0;
    let mut hi = 0;
    let mut sum = 0;
    let mut min_length = INF;

    while hi < n {
        if sum < s {
            hi += 1;
            sum += nums[hi - 1];
        } else {
            lo += 1;
            sum -= nums[lo - 1];
        }

        if sum >= s {
            let length = hi - lo;
            min_length = min_length.min(length);
        }
    }

    loop {
        lo += 1;
        sum -= nums[lo - 1];

        if sum < s {
            break;
        }

        let length = hi - lo;
        min_length = min_length.min(length);
    }

    if min_length == INF {
        min_length = 0;
    }

    println!("{min_length}");
}
