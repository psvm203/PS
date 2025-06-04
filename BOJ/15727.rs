use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let l: usize = stdin.trim().parse().unwrap();

    let time = (l + 4) / 5;

    println!("{time}");
}
