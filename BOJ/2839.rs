use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let n: usize = input.trim().parse().unwrap();

    if n == 4 || n == 7 {
        print!("-1");
        std::process::exit(0);
    }

    let bag_3_count = (0..).find(|bag_3| (n - bag_3 * 3) % 5 == 0).unwrap();
    let bag_5_count = (n - bag_3_count * 3) / 5;
    let bags_count = bag_3_count + bag_5_count;

    print!("{bags_count}");
}
