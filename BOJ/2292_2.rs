use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let n: f64 = input.trim().parse().unwrap();

    let dist = (((n - 1.0) / 3.0).sqrt() + 0.5) as usize + 1;

    print!("{dist}");
}
