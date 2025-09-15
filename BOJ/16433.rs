use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let nums: Vec<usize> = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect();

    let [n, r, c] = nums.try_into().unwrap();

    let output = if n % 2 == 0 {
        let unit = if (r + c) % 2 == 0 {
            "v.".repeat(n / 2) + "\n" + &".v".repeat(n / 2) + "\n"
        } else {
            ".v".repeat(n / 2) + "\n" + &"v.".repeat(n / 2) + "\n"
        };

        unit.repeat(n / 2)
    } else {
        let unit = if (r + c) % 2 == 0 {
            "v.".repeat(n / 2) + "v\n" + &".v".repeat(n / 2) + ".\n"
        } else {
            ".v".repeat(n / 2) + ".\n" + &"v.".repeat(n / 2) + "v\n"
        };

        let a = unit.repeat(n / 2 + 1);
        let mut binding = a.lines();
        binding.next_back();
        binding.collect::<Vec<_>>().join("\n")
    };

    println!("{output}");
}
