use std::io::*;

const INF: usize = 987654321;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let mut nums = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>());

    let mut read_int = || nums.next().unwrap();

    let n = read_int() * read_int();
    let mut blocks_count = read_int();

    let mut heights: Vec<_> = nums.collect();
    heights.sort_unstable_by(|a, b| b.cmp(a));

    let mut heights_sum: usize = heights.iter().sum();

    let mut cost = 0;
    let mut min_cost = INF;
    let mut result_height = 0;

    loop {
        let required_blocks = heights[0] * n - heights_sum;
        let fill_cost = cost + required_blocks;

        if required_blocks <= blocks_count && fill_cost < min_cost {
            min_cost = fill_cost;
            result_height = heights[0];
        }

        let target_height = heights[0];

        for height in &mut heights {
            if *height != target_height {
                break;
            }

            *height -= 1;
            heights_sum -= 1;
            blocks_count += 1;
            cost += 2;
        }

        if heights[0] == heights[n - 1] {
            if cost < min_cost {
                min_cost = cost;
                result_height = heights[0];
            }
            break;
        }
    }

    print!("{min_cost} {result_height}");
}
