use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let numbers = input.lines().flat_map(|x| x.parse::<i32>());

    let (index, max_number) = numbers
        .enumerate()
        .max_by_key(|&(_, number)| number)
        .unwrap();

    let index = index + 1;

    print!("{max_number}\n{index}");
}
