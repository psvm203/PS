use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let mut nums = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>());

    let n = nums.next().unwrap();

    let output = (0..n)
        .map(|_| {
            let v = nums.next().unwrap();
            let mut counts = [0; 1001];

            for _ in 0..v {
                let s = nums.next().unwrap();
                counts[s] += 1;
            }

            let mut max = 0;
            let mut result = 0;

            for i in 1..1001 {
                if counts[i] > max {
                    max = counts[i];
                    result = i;
                }
            }

            result.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
