use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut nums = stdin.split_whitespace().flat_map(|x| x.parse::<usize>());
    let a = nums.next().unwrap();
    let b = nums.next().unwrap();
    
    println!("{}", b % 10 * a);
    println!("{}", b / 10 % 10 * a);
    println!("{}", b / 100 * a);
    println!("{}", b * a);
}
