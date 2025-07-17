use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.parse().unwrap();

    let count = 2_u32.pow(n as u32);

    println!("{count}");
}
