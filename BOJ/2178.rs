use std::collections::VecDeque;
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

    let board: Vec<_> = lines.map(|line| line.chars().collect::<Vec<_>>()).collect();

    let dy = [-1, 0, 1, 0];
    let dx = [0, -1, 0, 1];

    let mut deque = VecDeque::from([(0, 0, 1)]);
    let mut visited = vec![vec![false; c]; r];
    visited[0][0] = true;

    while let Some((cy, cx, dist)) = deque.pop_front() {
        for dir in 0..4 {
            let ny = cy as i32 + dy[dir];
            let nx = cx as i32 + dx[dir];

            if ny == -1 || nx == -1 {
                continue;
            }

            let ny = ny as usize;
            let nx = nx as usize;

            if ny == r || nx == c {
                continue;
            }

            if ny == r - 1 && nx == c - 1 {
                print!("{}", dist + 1);
                return;
            }

            if board[ny][nx] == '0' {
                continue;
            }

            if !visited[ny][nx] {
                visited[ny][nx] = true;
                deque.push_back((ny, nx, dist + 1));
            }
        }
    }
}
