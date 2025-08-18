use std::io::*;

fn solve(heights: &Vec<usize>, index: usize, visited: usize, sum: usize) -> usize {
    if index == 9 {
        if visited.count_ones() == 7 && sum == 100 {
            return visited;
        }

        return 0;
    }

    let no = solve(heights, index + 1, visited, sum);

    let yes = solve(
        heights,
        index + 1,
        visited | (1 << index),
        sum + heights[index],
    );

    yes.max(no)
}

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let mut heights: Vec<usize> = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    heights.sort_unstable();

    let dwarves = solve(&heights, 0, 0, 0);

    let output = (0..9)
        .filter(|i| (1 << i) & dwarves != 0)
        .map(|x| heights[x].to_string())
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
