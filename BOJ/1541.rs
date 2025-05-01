use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let (plus, minus) = match input.split_once('-') {
        Some((a, b)) => (String::from(a), String::from(b)),
        None => (input, String::new()),
    };

    let answer = plus
        .trim()
        .split('+')
        .flat_map(|x| x.parse::<i32>())
        .sum::<i32>()
        - minus
            .trim()
            .split(['+', '-'])
            .flat_map(|x| x.parse::<i32>())
            .sum::<i32>();
    print!("{answer}");
}
