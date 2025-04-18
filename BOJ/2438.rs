use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let answer = (1..=n)
        .map(|x| "*".repeat(x))
        .collect::<Vec<String>>()
        .join("\n");

    print!("{answer}");
}
