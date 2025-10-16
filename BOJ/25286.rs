use std::io::*;

struct Date {
    year: usize,
    month: usize,
    day: usize,
}

impl Date {
    const MONTHS: [usize; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    fn is_leap_year(year: usize) -> bool {
        if year % 400 == 0 {
            true
        } else if year % 100 == 0 {
            false
        } else if year % 4 == 0 {
            true
        } else {
            false
        }
    }

    fn month_days(year: usize, month: usize) -> usize {
        if Self::is_leap_year(year) && month == 2 {
            29
        } else {
            Self::MONTHS[month]
        }
    }

    fn subtract(&self) -> Self {
        let mut year = self.year;
        let mut month = self.month;
        let mut day = self.day - 1;

        if day == 0 {
            month -= 1;
        }

        if month == 0 {
            year -= 1;
            month = 12;
            day += Self::month_days(year, month);
        } else {
            day += Self::month_days(year, month);
        }

        Self { year, month, day }
    }

    fn to_string(&self) -> String {
        format!("{} {} {}", self.year, self.month, self.day)
    }
}

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let nums: Vec<usize> = stdin
        .split_ascii_whitespace()
        .flat_map(|x| x.parse())
        .skip(1)
        .collect();

    let output = nums
        .chunks(2)
        .map(|chunk| {
            let year = chunk[0];
            let month = chunk[1];

            Date {
                year,
                month,
                day: 1,
            }
            .subtract()
            .to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
