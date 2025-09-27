use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let k = stdin.split_ascii_whitespace().skip(1).next().unwrap();

    let mut evens = 0;
    let mut odds = 0;

    for ch in k.chars() {
        match ch {
            '0' | '2' | '4' | '6' | '8' => evens += 1,
            _ => odds += 1,
        }
    }

    let output = if evens > odds {
        0
    } else if evens < odds {
        1
    } else {
        -1
    };

    println!("{output}");
}
