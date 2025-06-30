use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines();

    let test_cases_count: usize = lines.next().unwrap().parse().unwrap();

    let mut solve_test_case = || {
        let bytes: usize = lines.next().unwrap().parse().unwrap();
        let s = lines.next().unwrap().replace("I", "1").replace("O", "0");

        let mut output = String::new();

        for b in 0..bytes {
            let byte_str = &s[b * 8..b * 8 + 8];
            let byte = isize::from_str_radix(byte_str, 2).unwrap();
            let ch = byte as u8 as char;
            output.push(ch);
        }

        output
    };

    let output = (1..=test_cases_count)
        .map(|t| format!("Case #{t}: {}", solve_test_case()))
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
