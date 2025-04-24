use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let is_end = |x: usize| x.to_string().contains("666");

    let output = (666..).filter(|&x| is_end(x)).nth(n - 1).unwrap();

    print!("{output}");
}
