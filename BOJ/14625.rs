use std::convert::*;
use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let [mut h, mut m, th, tm, n] = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    let mut cnt = 0;

    loop {
        if h / 10 == n || h % 10 == n || m % 10 == n || m / 10 == n {
            cnt += 1;
        }

        if h == th && m == tm {
            break;
        }

        m += 1;

        if m >= 60 {
            h += 1;
            m -= 60;
        }
    }

    print!("{cnt}");
}
