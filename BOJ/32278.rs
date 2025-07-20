use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: i64 = stdin.trim().parse().unwrap();

    let output = if i16::MIN as i64 <= n && n <= i16::MAX as i64 {
        "short"
    } else if i32::MIN as i64 <= n && n <= i32::MAX as i64 {
        "int"
    } else {
        "long long"
    };

    println!("{output}");
}
