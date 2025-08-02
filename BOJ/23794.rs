use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.trim().parse().unwrap();

    println!("{}", "@".repeat(n + 2));
    print!("{}", format!("@{}@\n", " ".repeat(n)).repeat(n));
    println!("{}", "@".repeat(n + 2));
}
