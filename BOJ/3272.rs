use std::collections::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines().skip(1);

    let mut read_ints = || -> Vec<usize> {
        lines
            .next()
            .unwrap()
            .split(' ')
            .flat_map(|x| x.parse())
            .collect()
    };

    let nums = read_ints();
    let x = read_ints()[0];

    let mut counts = HashMap::new();

    for &num in &nums {
        counts
            .entry(num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut cases_count = 0;

    for &num in &nums {
        if num >= x {
            continue;
        }

        if num + num == x {
            continue;
        }

        let supplement = x - num;

        if let Some(count) = counts.get(&supplement) {
            cases_count += count;
        }
    }

    cases_count /= 2;

    if x % 2 == 0 {
        if let Some(count) = counts.get(&(x / 2)) {
            cases_count += count * (count - 1) / 2;
        }
    }

    println!("{cases_count}");
}
