use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines();

    let plain = lines.next().unwrap();
    let keys = lines.next().unwrap();
    let mut key = keys.chars().cycle();

    let output: String = plain
        .chars()
        .map(|ch| {
            if ch == ' ' {
                key.next();
                return ' ';
            }

            let ord = (25 + ch as u8 - key.next().unwrap() as u8) % 26 + 'a' as u8;

            ord as char
        })
        .collect();

    println!("{output}");
}
