use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let k: usize = stdin.trim().parse().unwrap();

    let output = "AKA".to_owned() + &"RAKA".repeat(k);

    println!("{output}");
}
