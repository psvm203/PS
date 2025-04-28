use std::io::*;

fn main() {
    let n: usize = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse().unwrap()
    };

    let mut mask = 0usize;
    let mut output = BufWriter::new(stdout());

    (0..n).for_each(|_| {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let query: Vec<&str> = input.split_ascii_whitespace().collect();

        let read_int = || -> usize { query[1].parse().unwrap() };

        match query[0] {
            "add" => mask |= 1 << read_int(),
            "remove" => mask &= !(1 << read_int()),
            "check" => {
                let contains = (mask & (1 << read_int()) != 0) as usize;
                writeln!(output, "{contains}").unwrap();
            }
            "toggle" => mask ^= 1 << read_int(),
            "all" => mask = (1 << 21) - 1,
            "empty" => mask = 0,
            _ => {}
        };
    });
}
