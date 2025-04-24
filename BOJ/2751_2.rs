use std::io::*;

fn main() {
    let mut counts = [false; 2000001];

    let input = read_to_string(stdin()).unwrap();

    let numbers = input.lines().skip(1).flat_map(|x| x.parse::<i32>());
    numbers.for_each(|x| counts[(x + 1000000) as usize] = true);

    let output = counts
        .iter()
        .enumerate()
        .filter(|&(_, exists)| *exists)
        .map(|(i, _)| (i as i32 - 1000000).to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
