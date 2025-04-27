use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut numbers = input.split_ascii_whitespace().flat_map(|x| x.parse());

    let n: i32 = numbers.next().unwrap();

    let mut numbers: Vec<_> = numbers.collect();
    numbers.sort_unstable();

    let mean = (numbers.iter().sum::<i32>() as f64 / n as f64).round() as i32;
    let median = numbers[(n / 2) as usize];

    let mode = {
        let mut counts = [0usize; 8001];
        let mut max_count = 0;
        let mut modes = Vec::new();

        numbers.iter().copied().for_each(|number| {
            let index = (number + 4000) as usize;
            counts[index] += 1;

            if counts[index] > max_count {
                max_count = counts[index];
                modes.clear();
                modes.push(number);
            } else if counts[index] == max_count {
                modes.push(number);
            }
        });

        if modes.len() == 1 {
            modes[0]
        } else {
            modes[1]
        }
    };

    let range = numbers[(n - 1) as usize] - numbers[0];

    print!("{mean}\n{median}\n{mode}\n{range}");
}
