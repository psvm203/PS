use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines();

    let mut read_ints = || -> Vec<usize> {
        lines
            .next()
            .unwrap()
            .split(' ')
            .flat_map(|x| x.parse())
            .collect()
    };

    let n = read_ints()[0];
    let soldiers = read_ints();

    let count: usize = soldiers.iter().map(|&soldier| soldier.min(n)).sum();

    println!("{count}");
}
