use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    
    let nums: Vec<usize> = stdin.split_whitespace().flat_map(|x| x.parse()).collect();
    let [a, b, c] = [nums[0], nums[1], nums[2]];
    let sum = (a + b) % c;
    let prod = a * b % c;
    
    println!("{sum}");
    println!("{sum}");
    println!("{prod}");
    println!("{prod}");
}