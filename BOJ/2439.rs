use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let create_line = |x| {
        let stars = "*".repeat(x);
        let line = format!("{:>n$}", stars);
        line
    };

    let answer = (1..=n)
        .map(|x| create_line(x))
        .collect::<Vec<_>>()
        .join("\n");

    print!("{answer}");
}
