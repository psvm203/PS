use std::convert::TryInto;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let mut read_ints = || -> Vec<usize> {
        lines
            .next()
            .unwrap()
            .split(' ')
            .flat_map(|x| x.parse())
            .collect()
    };

    let [r, c] = read_ints().try_into().unwrap();
    let board: Vec<Vec<_>> = (0..r).map(|_| read_ints()).collect();

    let shapes = [
        [[0, 0], [0, 1], [0, 2], [0, 3]],
        [[0, 0], [0, 1], [1, 0], [1, 1]],
        [[0, 0], [1, 0], [2, 0], [2, 1]],
        [[0, 0], [1, 0], [1, 1], [2, 1]],
        [[0, 0], [0, 1], [0, 2], [1, 1]],
    ];

    let place_shape = |y: usize, x: usize, shape: [[usize; 2]; 4]| {
        let mut max_score = 0;

        for &i in &[-1, 1] {
            for &j in &[-1, 1] {
                for &(dy, dx) in &[(0, 1), (1, 0)] {
                    let mut score = 0;

                    for k in 0..4 {
                        let ny = y as i32 + i * shape[k][dy] as i32;
                        let nx = x as i32 + j * shape[k][dx] as i32;

                        if ny < 0 || nx < 0 {
                            score = 0;
                            break;
                        }

                        let ny = ny as usize;
                        let nx = nx as usize;

                        if ny >= r || nx >= c {
                            score = 0;
                            break;
                        }

                        score += board[ny][nx];
                    }

                    max_score = max_score.max(score);
                }
            }
        }

        max_score
    };

    let mut max_score = 0;

    for y in 0..r {
        for x in 0..c {
            for &shape in &shapes {
                max_score = max_score.max(place_shape(y, x, shape));
            }
        }
    }

    print!("{max_score}");
}
