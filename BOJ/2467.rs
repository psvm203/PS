use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let mut read_ints = || -> Vec<i32> {
        lines
            .next()
            .unwrap()
            .split(' ')
            .flat_map(|x| x.parse())
            .collect()
    };

    let n = read_ints()[0] as usize;
    let liquids = read_ints();

    let mut a = -1;
    let mut b = -1;
    let mut min = 2087654321;

    for i in 0..n {
        let target = -liquids[i];

        let point = i + &liquids[i + 1..].partition_point(|&liquid| liquid <= target);

        if point != i {
            if (liquids[point] - target).abs() < min {
                min = (liquids[point] - target).abs();
                a = -target;
                b = liquids[point];
            }
        }

        if point + 1 < n {
            if (liquids[point + 1] - target).abs() < min {
                min = (liquids[point + 1] - target).abs();
                a = -target;
                b = liquids[point + 1];
            }
        }
    }

    print!("{a} {b}");
}
