use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut c = stdin.chars().next().unwrap();
    let mut dist = 0;

    for ch in "ILOVEYONSEI".chars() {
        dist += c.max(ch) as usize - c.min(ch) as usize;
        c = ch;
    }

    println!("{dist}");
}
