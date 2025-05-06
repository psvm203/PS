use std::convert::TryInto;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let [r, c] = lines
        .next()
        .unwrap()
        .split(' ')
        .flat_map(|x| x.parse::<usize>())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let board: Vec<_> = lines.collect();

    let (sy, sx) = board
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.chars().position(|ch| ch == 'I').map(|x| (y, x)))
        .unwrap();

    let mut stack = vec![(sy, sx)];

    let mut visited = vec![vec![false; c + 1]; r + 1];
    visited[sy][sx] = true;

    let dy = [-1, 0, 1, 0];
    let dx = [0, -1, 0, 1];

    let mut answer = 0;

    while let Some((y, x)) = stack.pop() {
        for i in 0..4 {
            let ny = y as i32 + dy[i];
            let nx = x as i32 + dx[i];

            if ny == -1 || ny == r as i32 || nx == -1 || nx == c as i32 {
                continue;
            }

            let ny = ny as usize;
            let nx = nx as usize;

            if visited[ny][nx] {
                continue;
            }

            if board[ny].chars().nth(nx).unwrap() == 'X' {
                continue;
            }

            visited[ny][nx] = true;
            stack.push((ny, nx));

            if board[ny].chars().nth(nx).unwrap() == 'P' {
                answer += 1;
            }
        }
    }

    if answer != 0 {
        print!("{answer}");
    } else {
        print!("TT");
    };
}
