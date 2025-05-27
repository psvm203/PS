use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let (a, b, c) = {
        let vec: Vec<usize> = stdin
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
            .collect();
        (vec[0], vec[1], vec[2])
    };

    let mut t = a * 60 + b + c;
    if t >= 24 * 60 {
        t -= 24 * 60;
    }

    let h = t / 60;
    let m = t % 60;

    println!("{h} {m}");
}
