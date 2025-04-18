use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let lines = input.lines();

    let create_line = |x: &str| {
        let mut split = x.split_ascii_whitespace();
        let repeat_count: usize = split.next().unwrap().parse().unwrap();

        let line: String = split
            .next()
            .unwrap()
            .chars()
            .map(|ch| ch.to_string().repeat(repeat_count))
            .collect();

        line
    };

    let answer = lines
        .skip(1)
        .map(|x| format!("{}", create_line(x)))
        .collect::<Vec<String>>()
        .join("\n");

    print!("{answer}");
}
