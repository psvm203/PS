use std::collections::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let cards: Vec<usize> = stdin
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse())
        .collect();

    let mut scores: HashMap<_, _> = cards.iter().map(|&card| (card, 0_i32)).collect();

    for &card in &cards {
        for i in (card * 2..=1000000).step_by(card) {
            if let Some(multiple) = scores.get_mut(&i) {
                *multiple -= 1;
                *scores.get_mut(&card).unwrap() += 1;
            }
        }
    }

    let output = cards
        .iter()
        .map(|card| scores.get(card).unwrap().to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{output}");
}
