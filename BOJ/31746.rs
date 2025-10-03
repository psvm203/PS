use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let n: usize = stdin.trim().parse().unwrap();

    let output = if n % 2 == 0 {
        "SciComLove"
    } else {
        "evoLmoCicS"
    };

    println!("{output}");
}
