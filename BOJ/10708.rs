use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines();

    let mut read_line = || -> Vec<usize> {
        lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
            .collect()
    };

    let n = read_line()[0];
    let _ = read_line();
    let targets = read_line();

    let mut scores = vec![0; n];

    for target in targets {
        let expects = read_line();

        for (i, expect) in expects.iter().enumerate() {
            if *expect == target {
                scores[i] += 1;
            } else {
                scores[target - 1] += 1;
            }
        }
    }

    let output = scores
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
