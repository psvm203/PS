use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let ch = stdin.trim().to_uppercase().chars().next().unwrap();

    let output = if ch == 'N' { "Naver D2" } else { "Naver Whale" };

    println!("{output}");
}
