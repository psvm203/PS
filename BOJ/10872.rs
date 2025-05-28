use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.trim().parse().unwrap();

    let factorial = (1..=n).try_fold(1, usize::checked_mul).unwrap();

    println!("{factorial}");
}
