use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let nums: Vec<usize> = stdin
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse())
        .collect();

    let output = nums
        .into_iter()
        .map(|num| {
            let mut sum = 0;
            let mut num = num;
            let mut even = false;

            loop {
                if num == 0 {
                    break;
                }

                let mut r = num % 10;

                if even {
                    r *= 2;
                }

                if r >= 10 {
                    r = r / 10 + r % 10;
                }

                sum += r;
                num /= 10;
                even ^= true;
            }

            if sum % 10 == 0 {
                "T"
            } else {
                "F"
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
