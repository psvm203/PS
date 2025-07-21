use std::collections::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let lines = stdin.split_ascii_whitespace().skip(1);

    let output = lines
        .map(|line| {
            let mut counts = HashMap::<&str, usize>::from([
                ("TTT", 0),
                ("TTH", 0),
                ("THT", 0),
                ("THH", 0),
                ("HTT", 0),
                ("HTH", 0),
                ("HHT", 0),
                ("HHH", 0),
            ]);

            for i in 0..38 {
                let s = &line[i..i + 3];
                counts.entry(s).and_modify(|count| *count += 1);
            }

            format!(
                "{} {} {} {} {} {} {} {}",
                counts["TTT"],
                counts["TTH"],
                counts["THT"],
                counts["THH"],
                counts["HTT"],
                counts["HTH"],
                counts["HHT"],
                counts["HHH"],
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
