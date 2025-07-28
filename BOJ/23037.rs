use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut n: usize = stdin.trim().parse().unwrap();

    let mut sum = 0;

    loop {
        sum += (n % 10).pow(5);
        n /= 10;

        if n == 0 {
            break;
        }
    }

    println!("{sum}");
}
