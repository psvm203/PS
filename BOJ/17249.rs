use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let splitted = stdin.trim().split("(^0^)");

    let output = splitted
        .map(|taebo| taebo.chars().filter(|ch| *ch == '@').count().to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{output}");
}
