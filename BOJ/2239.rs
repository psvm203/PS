use std::io::*;

fn get_subgrid(y: usize, x: usize) -> usize {
    (y / 3) * 3 + x / 3
}

fn solve_sudoku(
    board: &mut Vec<Vec<usize>>,
    rows: &mut [[bool; 10]; 9],
    columns: &mut [[bool; 10]; 9],
    subgrids: &mut [[bool; 10]; 9],
    index: usize,
) {
    if index == 81 {
        for i in 0..9 {
            for j in 0..9 {
                print!("{}", board[i][j]);
            }
            println!();
        }
        std::process::exit(0);
    }

    let y = index / 9;
    let x = index % 9;
    let subgrid = get_subgrid(y, x);

    if board[y][x] != 0 {
        solve_sudoku(board, rows, columns, subgrids, index + 1);
        return;
    }

    for num in 1..10 {
        if !rows[y][num] && !columns[x][num] && !subgrids[subgrid][num] {
            board[y][x] = num;
            rows[y][num] = true;
            columns[x][num] = true;
            subgrids[subgrid][num] = true;

            solve_sudoku(board, rows, columns, subgrids, index + 1);

            board[y][x] = 0;
            rows[y][num] = false;
            columns[x][num] = false;
            subgrids[subgrid][num] = false;
        }
    }
}

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let mut board: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|ch| ch as usize - '0' as usize).collect())
        .collect();

    let mut rows = [[false; 10]; 9];
    let mut columns = [[false; 10]; 9];
    let mut subgrids = [[false; 10]; 9];

    (0..9).for_each(|y| {
        (0..9).for_each(|x| {
            let num = board[y][x];
            rows[y][num] = true;
            columns[x][num] = true;
            subgrids[get_subgrid(y, x)][num] = true;
        });
    });

    solve_sudoku(&mut board, &mut rows, &mut columns, &mut subgrids, 0);
}
