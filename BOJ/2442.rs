use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.trim().parse().unwrap();

    let output = (0..n)
        .map(|i| format!("{}{}", " ".repeat(n - i - 1), "*".repeat(i * 2 + 1)))
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
