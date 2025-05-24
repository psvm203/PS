use std::io::*;

const INF: usize = 987654321;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let arrows: Vec<usize> = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .take_while(|&x| x != 0)
        .collect();

    let mut memo = [[INF; 5]; 5];
    memo[0][0] = 0;

    let calc_dist = |a: usize, b: usize| {
        if a == 0 {
            2
        } else if a == b {
            1
        } else if a.max(b) - a.min(b) == 2 {
            4
        } else {
            3
        }
    };

    for arrow in arrows {
        let mut next_memo = [[INF; 5]; 5];

        for i in 0..5 {
            if arrow == i {
                continue;
            }

            for j in 0..5 {
                let dist = calc_dist(j, arrow);
                next_memo[arrow][i] = next_memo[arrow][i].min(memo[j][i] + dist);
                next_memo[i][arrow] = next_memo[i][arrow].min(memo[i][j] + dist);
            }
        }

        memo = next_memo;
    }

    let min_cost = (0..5).flat_map(|i| memo[i]).min().unwrap();

    println!("{min_cost}");
}
