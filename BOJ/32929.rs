use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let x: usize = stdin.trim().parse().unwrap();

    let output = match x % 3 {
        0 => 'S',
        1 => 'U',
        _ => 'O',
    };

    println!("{output}");
}
