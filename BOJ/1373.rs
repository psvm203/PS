use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    let mut pos = 1;
    let mut sum = 0;

    for ch in stdin.trim().chars().rev() {
        sum += (ch as usize - '0' as usize) * pos;
        pos *= 2;

        if pos == 8 {
            pos = 1;
            output.push_str(&format!("{:o}", sum));
            sum = 0;
        }
    }

    if output.is_empty() || sum != 0 {
        output.push_str(&format!("{:o}", sum));
    }

    let output: String = output.chars().rev().collect();

    println!("{output}");
}
