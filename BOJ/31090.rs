use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let test_cases = stdin
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse());

    let solve_test_case = |test_case: usize| {
        if (test_case + 1) % (test_case % 100) == 0 {
            "Good"
        } else {
            "Bye"
        }
    };

    let output = test_cases
        .map(solve_test_case)
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
