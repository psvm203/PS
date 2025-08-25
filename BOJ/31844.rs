use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();

    let robot = stdin.chars().position(|x| x == '@').unwrap() as i32;
    let _box = stdin.chars().position(|x| x == '#').unwrap() as i32;
    let target = stdin.chars().position(|x| x == '!').unwrap() as i32;

    let output = if robot < _box && _box < target {
        target - robot - 1
    } else if target < _box && _box < robot {
        robot - target - 1
    } else {
        -1
    };

    println!("{output}");
}
