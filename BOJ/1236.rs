use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut splitted = stdin.split_ascii_whitespace();

    let mut read_next = || splitted.next().unwrap();

    let r: usize = read_next().parse().unwrap();
    let c: usize = read_next().parse().unwrap();
    let board: Vec<&str> = (0..r).map(|_| read_next()).collect();

    let mut horizontal_mask = 2_usize.pow(c as u32) - 1;
    let mut vertical_mask = 2_usize.pow(r as u32) - 1;

    for y in 0..r {
        for x in 0..c {
            let cell = board[y].chars().nth(x).unwrap();

            if cell == 'X' {
                horizontal_mask &= !(1 << x);
                vertical_mask &= !(1 << y);
            }
        }
    }

    let horizontal_count = horizontal_mask.count_ones();
    let vertical_count = vertical_mask.count_ones();
    let result = horizontal_count.max(vertical_count);

    println!("{result}");
}
