use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let lines = stdin.lines().skip(1);

    let mut total_credit = 0;

    let output = lines
        .map(|line| {
            let splitted: Vec<_> = line.split_ascii_whitespace().collect();
            let credit: usize = splitted[1].parse().unwrap();
            let grade = match splitted[2].chars().nth(0).unwrap() {
                'A' => 4.0,
                'B' => 3.0,
                'C' => 2.0,
                'D' => 1.0,
                _ => 0.0,
            };

            let grade = match splitted[2].chars().nth(1) {
                Some('+') => grade + 0.3,
                Some('-') => grade - 0.3,
                _ => grade,
            };

            total_credit += credit;
            credit as f64 * grade
        })
        .sum::<f64>()
        / total_credit as f64;

    println!("{:.2}", (output * 100.0).round() / 100.0);
}
