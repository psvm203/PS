use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let people: Vec<Vec<usize>> = input
        .lines()
        .skip(1)
        .map(|line| {
            line.split_ascii_whitespace()
                .flat_map(|x| x.parse())
                .collect()
        })
        .collect();

    let get_rank = |weight, height| {
        people
            .iter()
            .filter(|person| person[0] > weight && person[1] > height)
            .count()
            + 1
    };

    let output = people
        .iter()
        .map(|person| get_rank(person[0], person[1]).to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
