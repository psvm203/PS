use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let [d, h, w] = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect::<Vec<f64>>()
        .try_into()
        .unwrap();

    let k2 = d * d / (h * h + w * w);
    let k = k2.sqrt();
    let height = (h * k) as usize;
    let width = (w * k) as usize;

    println!("{height} {width}");
}
