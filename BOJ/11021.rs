use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines();

    let mut read_ints = || -> Vec<usize> {
        lines
            .next()
            .unwrap()
            .split(' ')
            .flat_map(|x| x.parse())
            .collect()
    };

    let n = read_ints()[0];

    let output = (1..=n)
        .map(|i| {
            let [a, b] = read_ints().try_into().unwrap();
            let sum = a + b;
            format!("Case #{i}: {sum}")
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
