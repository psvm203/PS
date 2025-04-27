use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let mut numbers = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse::<usize>());

    let n = numbers.next().unwrap();
    let cables = numbers;

    let mut lo = 1usize;
    let mut hi = i32::MAX as usize;
    let mut max_length = 0;

    let cut_cables = |length: usize| -> usize { cables.clone().map(|cable| cable / length).sum() };

    while lo <= hi {
        let md = lo + (hi - lo) / 2;

        if cut_cables(md) < n {
            hi = md - 1;
        } else if cut_cables(md) >= n {
            max_length = max_length.max(md);
            lo = md + 1;
        }
    }

    print!("{max_length}");
}
