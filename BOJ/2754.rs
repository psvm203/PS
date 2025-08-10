use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let grade = stdin.trim();

    let mut score = match grade.chars().next().unwrap() {
        'A' => 4.0,
        'B' => 3.0,
        'C' => 2.0,
        'D' => 1.0,
        _ => 0.0,
    };

    if let Some(offset) = grade.chars().nth(1) {
        match offset {
            '+' => score += 0.3,
            '-' => score -= 0.3,
            _ => {}
        }
    }

    println!("{:.1}", score);
}
