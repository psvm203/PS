use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let numbers = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.parse::<usize>());

    let mut cur = 1;
    let mut stack = Vec::new();
    let mut output = String::new();

    numbers.for_each(|number| {
        while cur <= number {
            stack.push(cur);
            cur += 1;
            output += "+\n";
        }

        if stack.last() == Some(&number) {
            stack.pop();
            output += "-\n";
        } else {
            print!("NO");
            std::process::exit(0);
        }
    });

    print!("{output}");
}
