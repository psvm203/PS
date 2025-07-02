use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.trim().parse().unwrap();

    let output = format!("{}int", "long ".repeat(n / 4));

    println!("{output}");
}
