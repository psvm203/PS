use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n = stdin.trim().parse().unwrap();

    for i in (1..=n).rev() {
        for _ in 0..i {
            print!("*");
        }
        println!();
    }
}
