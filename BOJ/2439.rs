use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let n: usize = input.trim().parse().unwrap();

    let create_line = |x| {
        let spaces = " ".repeat(n - x);
        let stars = "*".repeat(x);
        spaces + &stars
    };

    let answer = (1..=n)
        .map(|x| create_line(x))
        .collect::<Vec<String>>()
        .join("\n");

    print!("{answer}");
}
