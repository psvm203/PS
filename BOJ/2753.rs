use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let year: usize = input.trim().parse().unwrap();

    let is_leap_year = |year| {
        if year % 400 == 0 {
            true
        } else if year % 100 == 0 {
            false
        } else if year % 4 == 0 {
            true
        } else {
            false
        }
    };

    let answer = is_leap_year(year) as usize;

    print!("{answer}");
}
