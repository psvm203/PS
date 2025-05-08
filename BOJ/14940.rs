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

    let [r, c] = read_ints().try_into().unwrap();
    let r = r as usize;
    let c = c as usize;
    let mut board: Vec<_> = (0..r).map(|_| read_ints()).collect();

    let mut sy = 0;
    let mut sx = 0;

    for y in 0..r {
        for x in 0..c {
            if board[y][x] == 2 {
                board[y][x] = 0;
                sy = y;
                sx = x;
            } else if board[y][x] == 1 {
                board[y][x] = -1;
            }
        }
    }

    let dy = [-1, 0, 1, 0];
    let dx = [0, -1, 0, 1];
    let mut deque = VecDeque::from([(sy, sx)]);

    while let Some((cy, cx)) = deque.pop_front() {
        for i in 0..4 {
            let ny = cy as i32 + dy[i];
            let nx = cx as i32 + dx[i];

            if ny == -1 || ny == r as i32 || nx == -1 || nx == c as i32 {
                continue;
            }

            let ny = ny as usize;
            let nx = nx as usize;

            if board[ny][nx] == -1 {
                board[ny][nx] = board[cy][cx] + 1;
                deque.push_back((ny, nx));
            }
        }
    }

    let output = board
        .iter()
        .map(|row| {
            row.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
