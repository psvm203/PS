use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let nums: Vec<i32> = stdin
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse())
        .collect();

    let output = nums
        .chunks(2)
        .scan(0, |prev, chunk| {
            let current = &*prev + chunk[0] - chunk[1];
            *prev = current;
            Some(current.to_string())
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
