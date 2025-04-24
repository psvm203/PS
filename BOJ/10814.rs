use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let mut members: Vec<(usize, &str)> = input
        .lines()
        .skip(1)
        .map(|line| {
            let vec: Vec<&str> = line.split_ascii_whitespace().collect();
            (vec[0], vec[1])
        })
        .map(|(age, name)| (age.parse().unwrap(), name))
        .collect();

    let cmp = |a: &(usize, &str), b: &(usize, &str)| a.0.cmp(&b.0);

    members.sort_by(|a, b| cmp(a, b));

    let output = members
        .iter()
        .map(|(age, name)| format!("{age} {name}"))
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
