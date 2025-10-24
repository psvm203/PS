use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let bigdata = stdin.chars().filter(|ch| *ch == 'b').count();
    let security = stdin.chars().filter(|ch| *ch == 's').count();

    let mut output = String::new();

    if bigdata >= security {
        output.push_str("bigdata? ");
    }

    if security >= bigdata {
        output.push_str("security!");
    }

    println!("{output}");
}
