use std::io::*;

fn is_vowel(ch: char) -> bool {
    match ch {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let output = stdin.chars().filter(|ch| is_vowel(*ch)).count();

    println!("{output}");
}
