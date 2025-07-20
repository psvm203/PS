use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.trim().parse().unwrap();

    let output = if n < 100000 && n % 2024 == 0 {
        "Yes"
    } else {
        "No"
    };

    println!("{output}");
}
