use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let test_cases = stdin.split_ascii_whitespace().skip(1);

    let output = test_cases
        .map(|test_case| {
            let num: usize = test_case.parse().unwrap();
            let reversed: String = test_case.chars().rev().collect();
            let reversed_num: usize = reversed.parse().unwrap();
            let sum = num + reversed_num;

            let reversed_sum: usize = sum
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse()
                .unwrap();

            if sum == reversed_sum {
                "YES"
            } else {
                "NO"
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
