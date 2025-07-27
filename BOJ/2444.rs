use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.trim().parse().unwrap();

    for i in 0..n {
        println!("{}{}", " ".repeat(n - i - 1), "*".repeat(i * 2 + 1));
    }

    for i in (0..n - 1).rev() {
        println!("{}{}", " ".repeat(n - i - 1), "*".repeat(i * 2 + 1));
    }
}
