use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut splitted = stdin.split_ascii_whitespace();
    let level: usize = splitted.next().unwrap().parse().unwrap();
    let x = splitted.next().unwrap();

    let score = match x {
        "miss" => 0,
        "bad" => 200,
        "cool" => 400,
        "great" => 600,
        "perfect" => 1000,
        _ => 0,
    } * level;

    println!("{score}");
}
