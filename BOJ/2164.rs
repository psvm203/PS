use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut deque: VecDeque<usize> = (1..=n).collect();

    while deque.len() >= 2 {
        deque.pop_front();
        let card = deque.pop_front().unwrap();
        deque.push_back(card);
    }

    let last_card = deque.front().unwrap();

    print!("{last_card}");
}
