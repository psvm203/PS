use std::io::*;

fn cut_paper(paper: &Vec<Vec<usize>>, y: usize, x: usize, length: usize) -> [usize; 2] {
    let color = paper[y][x];

    if (0..length).all(|i| (0..length).all(|j| paper[y + i][x + j] == color)) {
        return match color {
            0 => [1, 0],
            1 => [0, 1],
            _ => unreachable!(),
        };
    }

    let next_length = length / 2;
    let mut counts = [0; 2];

    [
        cut_paper(paper, y, x, next_length),
        cut_paper(paper, y, x + next_length, next_length),
        cut_paper(paper, y + next_length, x, next_length),
        cut_paper(paper, y + next_length, x + next_length, next_length),
    ]
    .iter()
    .for_each(|[white_count, blue_count]| {
        counts[0] += white_count;
        counts[1] += blue_count;
    });

    counts
}

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

    let n = read_ints()[0];
    let paper: Vec<_> = (0..n).map(|_| read_ints()).collect();

    let [white_count, blue_count] = cut_paper(&paper, 0, 0, n);

    print!("{white_count}\n{blue_count}");
}
