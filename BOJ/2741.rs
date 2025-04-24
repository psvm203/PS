use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let output = (1..=n)
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
