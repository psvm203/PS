use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut words: Vec<&str> = input.lines().skip(1).collect();

    let cmp = |a: &str, b: &str| {
        if a.chars().count() == b.chars().count() {
            a.cmp(b)
        } else {
            a.chars().count().cmp(&b.chars().count())
        }
    };

    words.sort_by(|a, b| cmp(a, b));
    words.dedup();

    let output = words.join("\n");

    print!("{output}");
}
