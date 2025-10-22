use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let nums = stdin.split_ascii_whitespace().skip(1);

    let output = nums
        .map(|num| {
            if num
                .chars()
                .last()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap()
                % 2
                == 0
            {
                "even"
            } else {
                "odd"
            }
            .to_owned()
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
