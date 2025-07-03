use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut splitted = stdin.split_ascii_whitespace();

    let n: usize = splitted.next().unwrap().parse().unwrap();
    let s = splitted.next().unwrap();

    println!("{}", &s[n - 5..]);
}
