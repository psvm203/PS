use std::io::*;

fn main() {
    let n: usize = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse().unwrap()
    };

    let mut counts = [0; 10001];

    for _ in 0..n {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let number: usize = input.trim().parse().unwrap();
        counts[number] += 1;
    }

    let mut output = BufWriter::new(stdout());

    for (number, &count) in counts.iter().enumerate() {
        for _ in 0..count {
            write!(output, "{number}\n").unwrap();
        }
    }
}
