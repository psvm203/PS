use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let mut nums = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>());

    let n = nums.next().unwrap();
    let fruits: Vec<_> = nums.collect();

    let mut counts = [0; 10];
    let mut lo = 0;
    let mut hi = 0;
    let mut types = 0;
    let mut max_fruits_count = 0;

    loop {
        if types == 3 {
            counts[fruits[lo]] -= 1;

            if counts[fruits[lo]] == 0 {
                types -= 1;
            }

            lo += 1;
            continue;
        }

        let fruits_count = hi - lo;
        max_fruits_count = max_fruits_count.max(fruits_count);

        if hi == n {
            break;
        }

        hi += 1;
        counts[fruits[hi - 1]] += 1;

        if counts[fruits[hi - 1]] == 1 {
            types += 1;
        }
    }

    print!("{max_fruits_count}");
}
