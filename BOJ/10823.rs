use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let s = stdin.replace("\n", "");

    let sum: usize = s.split(',').flat_map(|x| x.parse::<usize>()).sum();

    println!("{sum}");
}
