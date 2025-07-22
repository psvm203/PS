use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let s = stdin.trim();

    let output: String = s
        .chars()
        .map(|ch| {
            if ch.is_ascii_uppercase() {
                ch.to_lowercase().collect::<Vec<_>>()
            } else {
                ch.to_uppercase().collect::<Vec<_>>()
            }
            .into_iter()
            .collect::<String>()
        })
        .collect();

    println!("{output}");
}
