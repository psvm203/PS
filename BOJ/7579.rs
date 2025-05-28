use std::cmp::*;
use std::convert::*;
use std::io::*;

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

    let [n, m] = read_ints().try_into().unwrap();
    let memories = read_ints();
    let costs = read_ints();

    let mut apps: Vec<_> = memories
        .iter()
        .zip(costs.iter())
        .map(|(&memory, &cost)| [memory, cost])
        .collect();

    apps.sort_unstable_by_key(|[memory, cost]| (*cost, Reverse(*memory)));

    let mut memo = vec![0; n * 100 + 1];
    let mut visited = vec![0i128; n * 100 + 1];

    for i in 0..n {
        if apps[i][1] != 0 {
            break;
        }

        memo[0] += apps[i][0];
        visited[0] |= 1 << i;
    }

    if memo[0] >= m {
        println!("0");
        return;
    }

    for cost in 1..=n * 100 {
        for prev_cost in 0.max(cost as i32 - 100) as usize..cost {
            let j = apps.partition_point(|&x| prev_cost + x[1] < cost);

            for i in j..n {
                if prev_cost + apps[i][1] > cost {
                    break;
                }

                if memo[prev_cost] + apps[i][0] > memo[cost] && (visited[prev_cost] & (1 << i)) == 0
                {
                    memo[cost] = memo[prev_cost] + apps[i][0];
                    visited[cost] = visited[prev_cost] | 1 << i;
                }
            }
        }

        if memo[cost] >= m {
            println!("{cost}");
            return;
        }
    }
}
