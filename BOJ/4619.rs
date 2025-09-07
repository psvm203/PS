use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let lines = stdin.lines().take_while(|&line| line != "0 0");

    let output = lines
        .map(|line| {
            let [b, n] = line
                .split_ascii_whitespace()
                .flat_map(|x| x.parse())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap();

            let candidate = (b as f64).powf(1.0 / (n as f64)).round() as usize;
            let candidates = [candidate - 1, candidate];
            let mut min_error = 987654321_i32;
            let mut result = 0;

            for a in candidates {
                let error = (b as i32 - a.pow(n as u32) as i32).abs();

                if error < min_error {
                    min_error = error;
                    result = a;
                }
            }

            result.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
