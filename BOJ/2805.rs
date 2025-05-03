use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let mut read_ints = || -> Vec<i128> {
        lines
            .next()
            .unwrap()
            .split(' ')
            .flat_map(|x| x.parse())
            .collect()
    };

    let m = read_ints()[1];
    let trees = read_ints();

    let mut lo = 1;
    let mut hi = 1e9 as i128;
    let mut output = 0;

    let cut_tree = |height| -> i128 { trees.iter().map(|tree| 0.max(tree - height)).sum() };

    while lo <= hi {
        let md = lo + (hi - lo) / 2;

        if cut_tree(md) >= m {
            output = md;
            lo = md + 1;
        } else {
            hi = md - 1;
        }
    }

    print!("{output}");
}
