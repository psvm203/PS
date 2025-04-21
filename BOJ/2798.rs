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

    let (n, maximum) = {
        let vec = read_ints();
        (vec[0], vec[1])
    };

    let mut cards = read_ints();
    cards.sort();

    let mut sum_max = 0;

    for i in 0..n - 1 {
        if cards[i] >= maximum {
            break;
        }

        for j in i + 1..n {
            if cards[i] + cards[j] >= maximum {
                break;
            }

            let target = maximum - cards[i] - cards[j];

            let k = match cards.binary_search(&target) {
                Ok(index) => index as i32,
                Err(index) => index as i32 - 1,
            };

            if k == -1 {
                break;
            }

            let k = k as usize;

            if j < k {
                sum_max = sum_max.max(cards[i] + cards[j] + cards[k]);
            }
        }
    }

    print!("{sum_max}");
}
