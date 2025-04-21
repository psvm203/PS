use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut maximum = 1;
    let mut diff = 6;

    for i in 1.. {
        if maximum >= n {
            print!("{i}");
            break;
        }

        maximum = maximum + diff;
        diff = diff + 6;
    }
}
