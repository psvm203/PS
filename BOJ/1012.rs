use std::collections::VecDeque;
use std::convert::TryInto;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let mut read_ints = || -> Vec<usize> {
        lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .flat_map(|x| x.parse())
            .collect()
    };

    let test_cases_count = read_ints()[0];

    let dy = [-1, 0, 1, 0];
    let dx = [0, -1, 0, 1];

    (0..test_cases_count).for_each(|_| {
        let [r, c, cabbages_count] = read_ints().try_into().unwrap();
        let cabbages = (0..cabbages_count).map(|_| read_ints());

        let mut is_vulnerable = vec![vec![false; c]; r];

        cabbages.for_each(|cabbage| {
            let (y, x) = (cabbage[0], cabbage[1]);
            is_vulnerable[y][x] = true;
        });

        let mut worms_count = 0;

        let mut place_worm = |y: usize, x: usize| {
            if !is_vulnerable[y][x] {
                return;
            }

            is_vulnerable[y][x] = false;
            worms_count += 1;

            let mut deque = VecDeque::from([(y, x)]);

            while let Some(cur) = deque.pop_front() {
                let (cy, cx) = cur;

                (0..4).for_each(|dir| {
                    let (ny, nx) = (cy as i32 + dy[dir], cx as i32 + dx[dir]);

                    if ny != -1
                        && ny != r as i32
                        && nx != -1
                        && nx != c as i32
                        && is_vulnerable[ny as usize][nx as usize]
                    {
                        is_vulnerable[ny as usize][nx as usize] = false;
                        deque.push_back((ny as usize, nx as usize));
                    }
                });
            }
        };

        (0..r).for_each(|y| {
            (0..c).for_each(|x| {
                place_worm(y, x);
            })
        });

        println!("{worms_count}");
    });
}
