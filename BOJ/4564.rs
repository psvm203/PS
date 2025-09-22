use std::io::*;

fn decompose(mut x: usize) -> Vec<usize> {
    let mut result = vec![x];

    while x >= 10 {
        x = x
            .to_string()
            .chars()
            .map(|ch| ch as usize - '0' as usize)
            .product();

        result.push(x);
    }

    result
}

fn vec_to_string(vec: Vec<usize>) -> String {
    vec.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let test_cases = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .take_while(|x| *x != 0);

    let output = test_cases
        .map(|test_case| vec_to_string(decompose(test_case)))
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
