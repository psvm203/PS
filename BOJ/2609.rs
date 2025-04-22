use std::io::*;

fn calc_gcd(a: usize, b: usize) -> usize {
    println!("{a} {b}");
    if b == 0 {
        a
    } else {
        calc_gcd(b, a % b)
    }
}

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let (a, b) = {
        let vec: Vec<usize> = input
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
            .collect();

        (vec[0], vec[1])
    };

    let gcd = calc_gcd(a, b);
    let lcm = a * b / gcd;

    print!("{gcd}\n{lcm}");
}
