use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mbti = stdin.trim();

    let output: String = mbti
        .chars()
        .map(|ch| match ch {
            'E' => 'I',
            'I' => 'E',
            'S' => 'N',
            'N' => 'S',
            'T' => 'F',
            'F' => 'T',
            'J' => 'P',
            _ => 'J',
        })
        .collect();

    println!("{output}");
}
