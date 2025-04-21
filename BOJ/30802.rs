use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut read_ints = || -> Vec<usize> {
        lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
            .collect()
    };

    let sizes = read_ints();

    let (t, p) = {
        let vec = read_ints();
        (vec[0], vec[1])
    };

    let tshirt_bundle: usize = sizes.iter().map(|size| (size + t - 1) / t).sum();
    let pen_bundle = n / p;
    let pen = n % p;

    print!("{tshirt_bundle}\n{pen_bundle} {pen}");
}
