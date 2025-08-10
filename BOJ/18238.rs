use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let s = stdin.trim();

    let mut state: u8 = 65;
    let mut cost: usize = 0;

    for ch in s.chars() {
        let min = state.min(ch as u8);
        let max = state.max(ch as u8);

        let a = max - min;
        let b = min + 26 - max;

        cost += a.min(b) as usize;
        state = ch as u8;
    }

    println!("{cost}");
}
