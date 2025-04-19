use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let score: usize = input.trim().parse().unwrap();

    let grade = if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else if score >= 60 {
        'D'
    } else {
        'F'
    };

    print!("{grade}");
}
