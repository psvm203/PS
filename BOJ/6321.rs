use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let lines: Vec<&str> = stdin.lines().skip(1).collect();

    let output = (0..lines.len())
        .map(|i| {
            let s: String = lines[i]
                .chars()
                .map(|ch| ((ch as u8 - 64) % 26 + 65) as char)
                .collect();
            format!("String #{}\n{s}", i + 1)
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    println!("{output}");
}
