use std::io::*;

fn contains(word: &str, chars: [char; 7]) -> bool {
    for ch in chars {
        if !word.contains(ch) {
            return false;
        }
    }

    true
}

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let word = stdin
        .split_ascii_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .to_owned();

    let lower = ['r', 'o', 'y', 'g', 'b', 'i', 'v'];
    let yes = contains(&word, lower);

    let upper = ['R', 'O', 'Y', 'G', 'B', 'I', 'V'];
    let YES = contains(&word, upper);

    let output = if yes && YES {
        "YeS"
    } else if yes {
        "yes"
    } else if YES {
        "YES"
    } else {
        "NO!"
    };

    println!("{output}");
}
