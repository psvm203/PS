use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();
    let board: Vec<Vec<_>> = lines.map(|line| line.chars().collect()).collect();

    let mut visited = vec![vec![false; n]; n];

    let dy = [-1, 0, 1, 0];
    let dx = [0, -1, 0, 1];

    let mut bfs = |y: usize, x: usize| {
        if board[y][x] == '0' || visited[y][x] {
            return 0;
        }

        visited[y][x] = true;
        let mut deque = VecDeque::from([(y, x)]);
        let mut area = 1;

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

                if board[ny][nx] == '0' || visited[ny][nx] {
                    continue;
                }

                visited[ny][nx] = true;
                deque.push_back((ny, nx));
                area += 1;
            }
        }

        area
    };

    let mut areas = Vec::new();

    (0..n).for_each(|y| {
        (0..n).for_each(|x| {
            let area = bfs(y, x);
            if area != 0 {
                areas.push(area);
            }
        })
    });

    areas.sort_unstable();

    let areas_len = areas.len();

    let output = areas
        .iter()
        .map(|area| area.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{areas_len}\n{output}");
}
