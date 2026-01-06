use std::io::*;
use std::cmp::max;

type Board = Vec<Vec<usize>>;

#[derive(PartialEq, Clone, Copy)]
struct Direction {
    dy: isize,
    dx: isize,
}

const DIRECTIONS: [Direction; 4] = [UP, DOWN, LEFT, RIGHT];
const UP: Direction = Direction {
    dy: -1,
    dx: 0,
};
const DOWN: Direction = Direction {
    dy: 1,
    dx: 0,
};
const LEFT: Direction = Direction {
    dy: 0,
    dx: -1,
};
const RIGHT: Direction = Direction {
    dy: 0,
    dx: 1,
};

fn shift_board(mut board: Board, direction: Direction) -> Board {
    let n = board.len() as isize;
    let in_range = |y: isize, x: isize| -> bool {
        0 <= y && y < n && 0 <= x && x < n
    };
    let is_vertical = |direction: Direction| -> bool {
        direction.dx == 0
    };

    let mut by = if direction == DOWN { n - 1 } else { 0 };
    let mut bx = if direction == RIGHT { n - 1 } else { 0 };
    while in_range(by, bx) {
        let mut nums = vec![];
        let mut cy = by;
        let mut cx = bx;
        while in_range(cy, cx) {
            let num = board[cy as usize][cx as usize];
            if num != 0 {
                nums.push(num);
            }
            cy -= direction.dy;
            cx -= direction.dx;
        }
        for i in 1..nums.len() {
            if nums[i - 1] == nums[i] {
                nums[i - 1] *= 2;
                nums[i] = 0;
            }
        }
        nums.retain(|&x| x != 0);
        nums.resize(n as usize, 0);
        cy = by;
        cx = bx;
        for num in nums {
            board[cy as usize][cx as usize] = num;
            cy -= direction.dy;
            cx -= direction.dx;
        }
        match is_vertical(direction) {
            true => {
                bx += 1;
            }
            false => {
                by += 1;
            }
        }
    }
    board
}

fn board_max(board: &Board) -> usize {
    board.iter().flatten().copied().max().unwrap()
}

fn dfs(board: Board, count: usize) -> usize {
    if count == 5 {
        return board_max(&board);
    }

    let mut mx = 0;
    for direction in DIRECTIONS {
        let next_board = shift_board(board.clone(), direction);
        let next = dfs(next_board, count + 1);
        mx = max(mx, next);
    }
    mx
}

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let board: Board = stdin.lines().skip(1).map(|line| line.split_whitespace().flat_map(|x| x.parse()).collect()).collect();

    let result = dfs(board, 0);
    println!("{result}");
}