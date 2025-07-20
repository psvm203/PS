use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let mut nums = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<i32>());

    let n = nums.next().unwrap() as usize;
    nums.next();
    let board: Vec<i32> = nums.clone().take(n).collect();
    let dice: Vec<i32> = nums.skip(n).collect();

    let mut cur = 0_usize;

    for i in 0.. {
        cur += dice[i] as usize;

        if cur >= n - 1 {
            println!("{}", i + 1);
            break;
        }

        cur = (cur as i32 + board[cur]) as usize;

        if cur >= n - 1 {
            println!("{}", i + 1);
            break;
        }
    }
}
