use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let (up, down, height) = {
        let vec: Vec<usize> = input
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
            .collect();

        (vec[0], vec[1], vec[2])
    };

    let days = (height - down - 1) / (up - down) + 1;

    print!("{days}");
}
