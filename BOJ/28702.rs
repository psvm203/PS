use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let lines = input.lines();

    let (i, line) = lines
        .enumerate()
        .find(|&(_, line)| line.parse::<usize>().is_ok())
        .unwrap();

    let fizz_buzz = |x: usize| {
        if x % 15 == 0 {
            String::from("FizzBuzz")
        } else if x % 3 == 0 {
            String::from("Fizz")
        } else if x % 5 == 0 {
            String::from("Buzz")
        } else {
            x.to_string()
        }
    };

    let output = fizz_buzz(line.parse::<usize>().unwrap() + 3 - i);

    print!("{output}");
}
