use std::collections::VecDeque;
use std::convert::TryInto;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let mut read_ints = || -> Vec<i32> {
        lines
            .next()
            .unwrap()
            .split(' ')
            .flat_map(|x| x.parse())
            .collect()
    };

    let [c, r] = read_ints().try_into().unwrap();

    let mut board: Vec<Vec<_>> = (0..r).map(|_| read_ints()).collect();

    let mut unripped_tomatoes_count = 0;
    let mut ripped_tomatoes = Vec::new();

    for y in 0..r as usize {
        for x in 0..c as usize {
            match board[y][x] {
                0 => unripped_tomatoes_count += 1,
                1 => ripped_tomatoes.push((y, x)),
                _ => {}
            }
        }
    }

    if unripped_tomatoes_count == 0 {
        print!("0");
        std::process::exit(0);
    }

    let dy = [-1, 0, 0, 1];
    let dx = [0, -1, 1, 0];

    let mut deque = VecDeque::from(ripped_tomatoes);

    for day in 1.. {
        let mut next_deque = VecDeque::new();

        while let Some((cy, cx)) = deque.pop_front() {
            for dir in 0..4 {
                let ny = cy as i32 + dy[dir];
                let nx = cx as i32 + dx[dir];

                if ny == -1 || ny == r || nx == -1 || nx == c {
                    continue;
                }

                let ny = ny as usize;
                let nx = nx as usize;

                if board[ny][nx] != 0 {
                    continue;
                }

                board[ny][nx] = 1;
                unripped_tomatoes_count -= 1;
                next_deque.push_back((ny, nx));
            }
        }

        deque = next_deque;

        if unripped_tomatoes_count == 0 {
            print!("{day}");
            break;
        }

        if deque.is_empty() {
            print!("-1");
            break;
        }
    }
}
