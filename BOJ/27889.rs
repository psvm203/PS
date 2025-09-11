use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let word = stdin.trim();

    let output = match word {
        "NLCS" => "North London Collegiate School",
        "BHA" => "Branksome Hall Asia",
        "KIS" => "Korea International School",
        _ => "St. Johnsbury Academy",
    };

    println!("{output}");
}
