use std::collections::HashMap;
use std::io::*;

fn fibonacci(memo: &mut HashMap<i32, usize>, x: i32) -> usize {
    if let Some(&result) = memo.get(&x) {
        result
    } else {
        let result = fibonacci(memo, x - 1) + fibonacci(memo, x - 2);
        memo.insert(x, result);
        result
    }
}

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let numbers = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse::<i32>());

    let mut memo: HashMap<i32, usize> = [(-1, 1), (0, 0)].iter().cloned().collect();

    let output = numbers
        .map(|number| {
            format!(
                "{} {}",
                fibonacci(&mut memo, number - 1),
                fibonacci(&mut memo, number)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
