use std::collections::HashSet;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let n: usize = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let unheard = lines.clone().take(n);
    let unseen = lines.skip(n);

    let set: HashSet<_> = unheard.collect();

    let mut unheard_and_unseen: Vec<_> = unseen.filter(|&x| set.contains(x)).collect();
    unheard_and_unseen.sort();

    let len = unheard_and_unseen.len();
    let output = unheard_and_unseen.join("\n");

    print!("{len}\n{output}");
}
