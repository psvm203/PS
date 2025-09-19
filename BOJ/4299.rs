use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let [sum, diff] = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    let a = (sum + diff) / 2;

    if sum < a {
        println!("-1");
        return;
    }

    let b = sum - a;

    let output = if a + b == sum && a - b == diff {
        format!("{a} {b}")
    } else {
        "-1".to_owned()
    };

    println!("{output}");
}
