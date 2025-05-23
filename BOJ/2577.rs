use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let product: usize = input.lines().flat_map(|x| x.parse::<usize>()).product();
    let product_string = product.to_string();

    let digits = product_string
        .chars()
        .flat_map(|x| x.to_digit(10))
        .map(|x| x as usize);

    let mut counts = [0; 10];

    for digit in digits {
        counts[digit] += 1;
    }

    let output = counts.map(|x| x.to_string()).join("\n");

    print!("{output}");
}
