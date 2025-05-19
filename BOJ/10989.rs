use std::io::*;

fn main() {
    let read_int = || -> usize {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse().unwrap()
    };

    let n = read_int();

    let mut counts = [0; 10001];

    for _ in 0..n {
        let num = read_int();
        counts[num] += 1;
    }

    let mut output = BufWriter::new(stdout());

    for (number, &count) in counts.iter().enumerate() {
        for _ in 0..count {
            write!(output, "{number}\n").unwrap();
        }
    }
}
