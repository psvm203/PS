use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let mut numbers = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>());

    let n = numbers.next().unwrap();
    let scores: Vec<usize> = numbers.collect();

    if n == 1 {
        let max_score = scores[0];
        print!("{max_score}");
        return;
    }

    // index, visited_prev
    let mut memo = vec![vec![0usize; 2]; n];
    memo[0][0] = scores[0];
    memo[1][0] = scores[1];
    memo[1][1] = scores[0] + scores[1];

    (2..n).for_each(|i| {
        memo[i][0] = memo[i - 2][0].max(memo[i - 2][1]) + scores[i];
        memo[i][1] = memo[i - 1][0] + scores[i];
    });

    let max_score = memo[n - 1][0].max(memo[n - 1][1]);

    print!("{max_score}");
}
