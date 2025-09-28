use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.trim().parse().unwrap();

    let output = format!(
        "{}{}",
        format!("{}\n", "@".repeat(n)).repeat(n * 4),
        format!("{}\n", "@".repeat(n * 5)).repeat(n)
    );

    print!("{output}");
}
