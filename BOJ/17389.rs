use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let table = stdin.split_ascii_whitespace().skip(1).next().unwrap();

    let mut score = 0;
    let mut bonus = 0;

    for (i, ch) in table.chars().enumerate() {
        if ch == 'O' {
            score += i + 1 + bonus;
            bonus += 1;
        } else {
            bonus = 0;
        }
    }

    println!("{score}");
}
