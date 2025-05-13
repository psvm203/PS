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
    let points: Vec<Vec<_>> = (0..n).map(|_| read_ints()).collect();

    let mut area = 0.0;

    for i in 0..n {
        let cur = &points[i];
        let next = &points[(i + 1) % n];

        area += cur[0] as f64 * next[1] as f64;
        area -= cur[1] as f64 * next[0] as f64;
    }

    area = area.abs() * 0.5;

    print!("{:.1}", area);
}
