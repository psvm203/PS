use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let test_cases = stdin.split_ascii_whitespace().skip(1);

    let output = test_cases
        .map(|x| {
            let a = x.chars().take_while(|&ch| ch == '!').count();
            let b_exists = x.chars().rev().next().unwrap() == '!';
            let mut n = x.chars().nth(a).unwrap().to_digit(10).unwrap();

            if b_exists {
                n = 1
            };

            if a % 2 == 1 {
                n ^= 1;
            }

            n.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
