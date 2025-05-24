use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines();

    let mut read_ints = || -> Vec<i64> {
        lines
            .next()
            .unwrap()
            .split(' ')
            .flat_map(|x| x.parse())
            .collect()
    };

    let n = read_ints()[0] as usize;
    let mut liquids = read_ints();
    liquids.sort();

    let mut min_diff: i64 = 10987654321;
    let mut result = [0i64; 3];

    for i in 0..n {
        for j in i + 1..n {
            let target = -liquids[i] - liquids[j];
            let point = j + &liquids[j + 1..].partition_point(|&liquid| liquid <= target);

            for &k in &[point, point + 1] {
                if j < k && k < n {
                    let diff = (liquids[i] + liquids[j] + liquids[k]).abs();

                    if diff < min_diff {
                        min_diff = diff;
                        result = [liquids[i], liquids[j], liquids[k]];
                    }
                }
            }
        }
    }

    let output = result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    print!("{output}");
}
