use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut splitted = stdin.split_ascii_whitespace();

    let now = splitted.next().unwrap();
    splitted.next();

    let output = splitted.filter(|x| now <= *x).count();

    println!("{output}");
}
