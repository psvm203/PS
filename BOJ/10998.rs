use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let product: usize = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .product();

    print!("{product}");
}
