use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let score: usize = input.trim().parse().unwrap();

    let grade = match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    };

    print!("{grade}");
}
