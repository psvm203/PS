use std::io::*;

fn rotate(x: usize) -> usize {
    if x < 10 {
        return x;
    }

    format!("{}{}", x % 10, x / 10).parse().unwrap()
}

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.trim().parse().unwrap();

    let mut rotated = n;
    let mut sum = 0;

    loop {
        rotated = rotate(rotated);
        sum += rotated;

        if rotated == n {
            break;
        }
    }

    println!("{sum}");
}
