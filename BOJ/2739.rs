use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let create_line = |x| {
        let product = n * x;
        let format = format!("{n} * {x} = {product}");

        format
    };

    let answer = (1..10)
        .map(|x| create_line(x))
        .collect::<Vec<String>>()
        .join("\n");

    print!("{answer}");
}
