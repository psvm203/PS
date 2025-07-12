use std::io::*;

fn main() {
	let stdin = read_to_string(stdin()).unwrap();
	let lines = stdin.lines();

    let split = |s: &str| -> (String, usize, usize) {
        let splitted: Vec<_> = s.split_ascii_whitespace().collect();
        (
            splitted[0].to_owned(),
            splitted[1].parse().unwrap(),
            splitted[2].parse().unwrap(),
        )
    };

    let process = |x: (String, usize, usize)| -> String {
        let age = if x.1 > 17 || x.2 >= 80 {
            "Senior"
        } else {
            "Junior"
        };

        format!("{} {age}", x.0)
    };

    let output = lines
        .map(split)
        .take_while(|splitted| splitted.1 != 0)
        .map(process)
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
