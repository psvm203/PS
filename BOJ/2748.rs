use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.trim().parse().unwrap();

    let mut a: usize = 0;
    let mut b: usize = 1;

    for _ in 0..n - 1 {
        let fibonacci = a + b;
        a = b;
        b = fibonacci;
    }

    println!("{b}");
}
