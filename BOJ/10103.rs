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
    let mut a = 100;
    let mut b = 100;

    for _ in 0..n {
        let dice = read_ints();

        if dice[0] < dice[1] {
            a -= dice[1];
        } else if dice[0] > dice[1] {
            b -= dice[0];
        }
    }

    println!("{a} {b}");
}
