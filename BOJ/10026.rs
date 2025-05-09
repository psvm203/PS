use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let mut read_line = || lines.next().unwrap();

    let n: usize = read_line().parse().unwrap();
    let board: Vec<Vec<_>> = (0..n).map(|_| read_line().chars().collect()).collect();

    let dy = [-1, 0, 1, 0];
    let dx = [0, -1, 0, 1];

    let mut visited = vec![vec![false; n]; n];
    let mut areas_count = 0;

    for y in 0..n {
        for x in 0..n {
            if visited[y][x] {
                continue;
            }
            areas_count += 1;

            let color = board[y][x];
            let mut deque = VecDeque::from([(y, x)]);

            while let Some((cy, cx)) = deque.pop_front() {
                for dir in 0..4 {
                    let ny = cy as i32 + dy[dir];
                    let nx = cx as i32 + dx[dir];

                    if ny == -1 || nx == -1 {
                        continue;
                    }

                    let ny = ny as usize;
                    let nx = nx as usize;

                    if ny == n || nx == n {
                        continue;
                    }

                    if board[ny][nx] != color {
                        continue;
                    }

                    if visited[ny][nx] {
                        continue;
                    }

                    visited[ny][nx] = true;
                    deque.push_back((ny, nx));
                }
            }
        }
    }

    print!("{areas_count} ");

    let mut visited = vec![vec![false; n]; n];
    let mut areas_count = 0;

    for y in 0..n {
        for x in 0..n {
            if visited[y][x] {
                continue;
            }
            areas_count += 1;

            let color = board[y][x];
            let mut deque = VecDeque::from([(y, x)]);

            while let Some((cy, cx)) = deque.pop_front() {
                for dir in 0..4 {
                    let ny = cy as i32 + dy[dir];
                    let nx = cx as i32 + dx[dir];

                    if ny == -1 || nx == -1 {
                        continue;
                    }

                    let ny = ny as usize;
                    let nx = nx as usize;

                    if ny == n || nx == n {
                        continue;
                    }

                    if (color == 'R' || color == 'G')
                        && (board[ny][nx] == 'R' || board[ny][nx] == 'G')
                        || (color == board[ny][nx])
                    {
                        if visited[ny][nx] {
                            continue;
                        }

                        visited[ny][nx] = true;
                        deque.push_back((ny, nx));
                    }
                }
            }
        }
    }

    print!("{areas_count}");
}
