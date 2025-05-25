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

    let n = read_ints()[0] as usize;
    let costs: Vec<Vec<_>> = (0..n).map(|_| read_ints()).collect();

    // index, color, beginning color
    let mut memo = vec![[[INF; 3]; 3]; n];

    for color in 0..3 {
        memo[0][color][color] = costs[0][color];
    }

    for i in 1..n - 1 {
        for color in 0..3 {
            for beginning_color in 0..3 {
                for prev_color in (0..3).filter(|&c| c != color) {
                    memo[i][color][beginning_color] = memo[i][color][beginning_color]
                        .min(memo[i - 1][prev_color][beginning_color] + costs[i][color]);
                }
            }
        }
    }

    for color in 0..3 {
        for beginning_color in (0..3).filter(|&bc| bc != color) {
            for prev_color in (0..3).filter(|&c| c != color) {
                memo[n - 1][color][beginning_color] = memo[n - 1][color][beginning_color]
                    .min(memo[n - 2][prev_color][beginning_color] + costs[n - 1][color]);
            }
        }
    }

    let mut min_cost = INF;

    for color in 0..3 {
        for beginning_color in 0..3 {
            min_cost = min_cost.min(memo[n - 1][color][beginning_color]);
        }
    }

    println!("{min_cost}");
}
