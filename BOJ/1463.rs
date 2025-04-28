use std::io::*;

const INF: usize = 987654321;

fn operate(x: usize, memo: &mut Vec<usize>) -> usize {
    if x == 1 {
        return 0;
    }

    if memo[x] != INF {
        return memo[x];
    }

    let mut min_operations = INF;

    if x % 3 == 0 {
        min_operations = operate(x / 3, memo) + 1;
    }

    if x % 2 == 0 {
        min_operations = min_operations.min(operate(x / 2, memo) + 1);
    }

    min_operations = min_operations.min(operate(x - 1, memo) + 1);
    memo[x] = min_operations;

    min_operations
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut memo = vec![INF; n + 1];

    let output = operate(n, &mut memo);

    print!("{output}");
}
