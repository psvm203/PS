use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let magnet = stdin.split_ascii_whitespace().nth(1).unwrap();

    for i in 1..magnet.len() {
        if magnet.chars().nth(i - 1) == magnet.chars().nth(i) {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
