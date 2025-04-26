use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let (n, k) = {
        let vec: Vec<usize> = input
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
            .collect();

        (vec[0], vec[1])
    };

    let mut last_dead_person = -1;
    let mut is_alive = vec![true; n];

    let mut kill_person = || {
        let dead_person = ((last_dead_person + 1) as usize..)
            .map(|x| x % n)
            .filter(|&x| is_alive[x])
            .nth(k - 1)
            .unwrap();

        last_dead_person = dead_person as i32;
        is_alive[dead_person] = false;

        last_dead_person + 1
    };

    let output = (0..n)
        .map(|_| kill_person().to_string())
        .collect::<Vec<_>>()
        .join(", ");

    print!("<{output}>");
}
