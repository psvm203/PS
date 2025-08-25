use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let words = stdin.split_ascii_whitespace().take_while(|x| *x != "#");

    let is_vowel = |ch| ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u';

    let output = words
        .map(|word| match word.chars().position(is_vowel) {
            Some(idx) => format!("{}{}ay", &word[idx..], &word[..idx]),
            None => format!("{word}ay"),
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
