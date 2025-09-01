use std::io::*;

fn solve(s: String) -> Option<usize> {
    let a = s.chars().position(|c| c == '2')?;
    let b = s.chars().skip(a).position(|c| c == '0')?;
    let c = s.chars().skip(a + b).position(|c| c == '2')?;
    s.chars().skip(a + b + c).position(|c| c == '3')?;
    Some(1)
}

fn is_winning_ticket(x: &usize) -> bool {
    solve(x.to_string()).is_some()
}

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.trim().parse().unwrap();

    let output = (2023..=n).filter(is_winning_ticket).count();

    println!("{output}");
}
