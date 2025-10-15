use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let word = stdin.trim();

    let output = match word {
        "SONGDO" => "HIGHSCHOOL",
        "CODE" => "MASTER",
        "2023" => "0611",
        _ => "CONTEST",
    };

    println!("{output}");
}
